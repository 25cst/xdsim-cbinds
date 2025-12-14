#pragma once

#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>


typedef enum Direction {
  Direction_Right,
  Direction_Up,
  Direction_Left,
  Direction_Down,
} Direction;

typedef enum ComponentType {
  ComponentType_Gate,
  ComponentType_Connection,
  ComponentType_Data,
} ComponentType;

typedef enum MenuInputIntegerStyle {
  MenuInputIntegerStyle_Slider,
  MenuInputIntegerStyle_TextBox,
  MenuInputIntegerStyle_SliderAndTextBox,
} MenuInputIntegerStyle;

typedef enum MenuInputFloatStyle {
  MenuInputFloatStyle_Slider,
  MenuInputFloatStyle_TextBox,
  MenuInputFloatStyle_SliderAndTextBox,
} MenuInputFloatStyle;

typedef enum MenuInputBooleanStyle {
  MenuInputBooleanStyle_CheckBox,
} MenuInputBooleanStyle;

/**
 * A non-resizeable, null-terminated string
 */
typedef struct Str {
  const char *first;
  void (*drop)(const char*);
} Str;

typedef struct PackageIdent {
  struct Str name;
  uint16_t major;
  uint16_t minor;
  uint16_t patch;
} PackageIdent;

/**
 * Information for a Connection
 */
typedef struct ConnectionDefinition {
  /**
   * Definition schema version number
   */
  uint32_t version;
  /**
   * Data type the connection carries
   */
  struct PackageIdent data_type;
  /**
   * Connection identifier: the unique identifier for the connection type
   * filled in by macro
   */
  struct PackageIdent identifier;
} ConnectionDefinition;

typedef void *ConnectionMut;

/**
 * A non-resizeable array with length
 */
typedef struct Slice {
  const void *first;
  uint64_t length;
  void (*drop)(const void*, uint64_t);
} Slice;

typedef struct Graphic {
  /**
   * [ Element ]
   */
  struct Slice elements;
} Graphic;

typedef const void *Connection;

/**
 * if is NONE, then write NULL
 */
typedef struct ConnectionJunction {
  const struct ConnectionSegment *up;
  const struct ConnectionSegment *right;
  const struct ConnectionSegment *down;
  const struct ConnectionSegment *left;
} ConnectionJunction;

typedef struct ConnectionSegment {
  /**
   * [ Vec2 ]
   */
  struct Slice path;
  /**
   * if is NONE, then write NULL
   */
  const struct ConnectionJunction *next;
} ConnectionSegment;

typedef const void *Data;

/**
 * Details of a request to draw a `Connection`
 */
typedef struct ConnectionDrawRequest {
  /**
   * Path the connection takes
   */
  const struct ConnectionSegment *path;
  /**
   * Current value in the connection
   */
  Data data;
} ConnectionDrawRequest;

typedef void *PropertiesMut;

typedef struct Vec2 {
  float x;
  float y;
} Vec2;

/**
 * TODO: this will need to have a stable byte structure
 * probably need to tag repr(C) or something
 * I need to read the nomicon
 */
typedef struct GateDefinition {
  /**
   * The ordered input that the gate takes
   * [ GateIOEntry ]
   */
  struct Slice inputs;
  /**
   * The ordered output that the gate produces
   * [ GateIOEntry ]
   */
  struct Slice outputs;
  /**
   * The visual bounding box (dimension) of the gate
   * The bottom left corner is (0, 0), top right corner is (width, height)
   */
  struct Vec2 bounding_box;
  /**
   * Gate identifier: the unique identifier for the gate type
   * filled in by macro
   */
  struct PackageIdent identifier;
} GateDefinition;

typedef const void *Gate;

typedef void *GateMut;

/**
 * A single gate draw request
 */
typedef struct GateDrawRequest {
  /**
   * One of the four the gate is facing (rotation)
   */
  enum Direction direction;
  /**
   * The size of the bounding box previously provided
   */
  struct Vec2 dimension;
} GateDrawRequest;

/**
 * A single gate tick request
 */
typedef struct GateTickRequest {
  /**
   * Inputs to the gate
   * [ *const Data ]
   */
  struct Slice inputs;
} GateTickRequest;

typedef struct PackageDefinition {
  struct PackageIdent ident;
  enum ComponentType component_type;
  /**
   * [ Str ]
   */
  struct Slice authors;
  struct Str description;
  struct Str homepage;
} PackageDefinition;

typedef const void *Properties;

/**
 * A menu is a list of MenuItems
 */
typedef struct Menu {
  /**
   * [ MenuItem ]
   */
  struct Slice items;
} Menu;

typedef enum MenuInputValue_Tag {
  MenuInputValue_String,
  MenuInputValue_Integer,
  MenuInputValue_Float,
  MenuInputValue_Boolean,
} MenuInputValue_Tag;

typedef struct MenuInputValue {
  MenuInputValue_Tag tag;
  union {
    struct {
      struct Str string;
    };
    struct {
      int64_t integer;
    };
    struct {
      double float_;
    };
    struct {
      bool boolean;
    };
  };
} MenuInputValue;

typedef enum PropertiesSetError_Tag {
  PropertiesSetError_NotExist,
  PropertiesSetError_Invalid,
} PropertiesSetError_Tag;

typedef struct PropertiesSetError {
  PropertiesSetError_Tag tag;
  union {
    struct {
      struct Str invalid;
    };
  };
} PropertiesSetError;

typedef enum MenuInputStringStyle_Tag {
  MenuInputStringStyle_Inline,
  MenuInputStringStyle_Multiline,
  MenuInputStringStyle_Dropdown,
} MenuInputStringStyle_Tag;

typedef struct MenuInputStringStyle_Inline_Body {
  struct Str placeholder;
} MenuInputStringStyle_Inline_Body;

typedef struct MenuInputStringStyle_Multiline_Body {
  uint32_t max_lines;
  struct Str placeholder;
} MenuInputStringStyle_Multiline_Body;

typedef struct MenuInputStringStyle_Dropdown_Body {
  /**
   * [ Str ]
   */
  struct Slice options;
} MenuInputStringStyle_Dropdown_Body;

typedef struct MenuInputStringStyle {
  MenuInputStringStyle_Tag tag;
  union {
    MenuInputStringStyle_Inline_Body inline_;
    MenuInputStringStyle_Multiline_Body multiline;
    MenuInputStringStyle_Dropdown_Body dropdown;
  };
} MenuInputStringStyle;

typedef enum MenuInput_Tag {
  MenuInput_String,
  MenuInput_Integer,
  MenuInput_Float,
  MenuInput_Boolean,
} MenuInput_Tag;

typedef struct MenuInput_String_Body {
  struct MenuInputStringStyle style;
} MenuInput_String_Body;

typedef struct MenuInput_Integer_Body {
  int64_t min;
  int64_t max;
  enum MenuInputIntegerStyle style;
} MenuInput_Integer_Body;

typedef struct MenuInput_Float_Body {
  double min;
  double max;
  enum MenuInputFloatStyle style;
} MenuInput_Float_Body;

typedef struct MenuInput_Boolean_Body {
  enum MenuInputBooleanStyle style;
} MenuInput_Boolean_Body;

typedef struct MenuInput {
  MenuInput_Tag tag;
  union {
    MenuInput_String_Body string;
    MenuInput_Integer_Body integer;
    MenuInput_Float_Body float_;
    MenuInput_Boolean_Body boolean;
  };
} MenuInput;

typedef enum MenuItemVariant_Tag {
  /**
   * Can be fold/unfold to hide/reveal menu items
   */
  MenuItemVariant_Foldable,
  MenuItemVariant_Text,
  MenuItemVariant_Input,
} MenuItemVariant_Tag;

typedef struct MenuItemVariant_Foldable_Body {
  struct Str title;
  /**
   * [ MenuItem ]
   */
  struct Slice items;
} MenuItemVariant_Foldable_Body;

typedef struct MenuItemVariant_Text_Body {
  struct Str content;
} MenuItemVariant_Text_Body;

typedef struct MenuItemVariant_Input_Body {
  struct Str id;
  struct MenuInput input_type;
} MenuItemVariant_Input_Body;

typedef struct MenuItemVariant {
  MenuItemVariant_Tag tag;
  union {
    MenuItemVariant_Foldable_Body foldable;
    MenuItemVariant_Text_Body text;
    MenuItemVariant_Input_Body input;
  };
} MenuItemVariant;

typedef struct MenuItem {
  struct Str tooltip;
  struct MenuItemVariant item_type;
} MenuItem;

/**
 * Representing a single input or output connection that the gate take.
 * - name: the unique name of the input/output
 * - data_type: the type name of the input/output
 * - position: a point that is on the bounding box
 */
typedef struct GateIOEntry {
  struct Str name;
  struct PackageIdent data_type;
  struct Vec2 position;
} GateIOEntry;

/**
 * Enum representing the colour options available for gates
 */
typedef enum Colour_Tag {
  /**
   * The primary colour of the gates (in a light theme, this would be black)
   */
  Colour_Fg,
  /**
   * The background colour (in a light theme, this would be white)
   */
  Colour_Bg,
  /**
   * Success colour (e.g. counter output in denary)
   */
  Colour_Success,
  /**
   * Info colour (e.g. counter output in denary)
   */
  Colour_Info,
  /**
   * Warn colour (e.g. potential incorrect operation)
   */
  Colour_Warn,
  /**
   * Error colour (e.g. a FSM not enabled)
   */
  Colour_Error,
  Colour_Black,
  Colour_Blue,
  Colour_Cyan,
  Colour_Green,
  Colour_Grey,
  Colour_Magenta,
  Colour_Red,
  Colour_White,
  Colour_Yellow,
  Colour_Transparent,
  /**
   * Arbitrary RGBA colour
   */
  Colour_Rgba,
  /**
   * A named colour from a palette
   *
   * A palette is a customisable set of colours
   * multiple gates/connections can share the same palette
   */
  Colour_Named,
} Colour_Tag;

typedef struct Colour_Rgba_Body {
  float r;
  float g;
  float b;
  float a;
} Colour_Rgba_Body;

typedef struct Colour_Named_Body {
  /**
   * Name of the palette
   */
  struct Str palette;
  /**
   * Name of the colour within the palette
   */
  struct Str name;
} Colour_Named_Body;

typedef struct Colour {
  Colour_Tag tag;
  union {
    Colour_Rgba_Body rgba;
    Colour_Named_Body named;
  };
} Colour;

typedef struct StrokeStyle {
  struct Colour colour;
} StrokeStyle;

typedef struct FillStyle {
  struct Colour colour;
} FillStyle;

typedef enum Element_Tag {
  Element_Line,
  Element_Rect,
} Element_Tag;

typedef struct Element_Line_Body {
  /**
   * [ Vec2 ]
   */
  struct Slice points;
  struct StrokeStyle stroke;
} Element_Line_Body;

typedef struct Element_Rect_Body {
  struct Vec2 pos;
  struct Vec2 size;
  struct StrokeStyle stroke;
  struct FillStyle fill;
} Element_Rect_Body;

typedef struct Element {
  Element_Tag tag;
  union {
    Element_Line_Body line;
    Element_Rect_Body rect;
  };
} Element;

extern const struct ConnectionDefinition *conn_def(void);

/**
 * You must not store the pointer to the slice, the slice will be dropped
 * You must malloc for the struct manually
 */
extern ConnectionMut conn_deserialize(struct Slice bytes);

extern struct Graphic conn_draw(Connection conn, const struct ConnectionDrawRequest *request);

extern void conn_drop(ConnectionMut conn);

extern PropertiesMut conn_props(ConnectionMut conn);

extern struct Slice conn_serialize(Connection conn);

/**
 * You must not store the pointer to the slice, the slice will be dropped
 * You must malloc for the struct manually
 */
extern Data data_deserialize(struct Slice bytes);

extern void data_drop(Data data);

extern struct Slice data_serialize(Data data);

extern struct GateDefinition gate_def(Gate gate);

extern GateMut gate_deserialize(struct Slice bytes);

extern struct Graphic gate_draw(Gate gate, const struct GateDrawRequest *request);

extern PropertiesMut gate_props(GateMut gate);

extern struct Slice gate_serialize(Gate gate);

extern struct Slice gate_tick(GateMut gate, const struct GateTickRequest *request);

extern struct PackageDefinition package_definition(void);

/**
 * You must not store the pointer to the slice, the slice will be dropped
 * You must malloc for the struct manually
 * Return NULL if deserialisation failed
 */
extern Properties props_deserialize(struct Slice bytes);

extern struct Menu props_get_menu(Properties props);

extern struct MenuInputValue props_get_option(Properties props, const struct Str *id);

/**
 * props will be dropped after passing to it
 * slice must not store pointers to it
 */
extern struct Slice props_serialize(Properties props);

/**
 * ID and value will be dropped after passing to it
 * props must not store the pointers to them
 * return NULL if no errors
 */
extern const struct PropertiesSetError *props_set_option(PropertiesMut props,
                                                         const struct Str *id,
                                                         const struct MenuInputValue *value);
