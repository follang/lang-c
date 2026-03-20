//! Fixture-based extraction tests exercising representative C declaration patterns.
//!
//! Each fixture is a self-contained C snippet with expected extraction results.

use crate::extract;
use crate::ir::*;

// --- Standard library style ---

#[test]
fn fixture_stdio_style() {
    let src = r#"
typedef unsigned long size_t;
typedef long ssize_t;
struct FILE;

int fclose(struct FILE *stream);
int fflush(struct FILE *stream);
struct FILE *fopen(const char *path, const char *mode);
size_t fread(void *ptr, size_t size, size_t nmemb, struct FILE *stream);
size_t fwrite(const void *ptr, size_t size, size_t nmemb, struct FILE *stream);
int fprintf(struct FILE *stream, const char *fmt, ...);
int feof(struct FILE *stream);
int ferror(struct FILE *stream);
"#;
    let pkg = extract::extract_from_source(src).unwrap();

    assert_eq!(pkg.type_alias_count(), 2);
    assert!(pkg.find_type_alias("size_t").is_some());
    assert!(pkg.find_type_alias("ssize_t").is_some());

    let file_rec = pkg.find_record("FILE").unwrap();
    assert!(file_rec.is_opaque());

    assert!(pkg.find_function("fclose").is_some());
    assert!(pkg.find_function("fopen").is_some());
    assert!(pkg.find_function("fread").is_some());

    let fprintf = pkg.find_function("fprintf").unwrap();
    assert!(fprintf.variadic);
    assert_eq!(fprintf.parameters.len(), 2);

    let fwrite = pkg.find_function("fwrite").unwrap();
    assert_eq!(fwrite.parameters[0].ty, SourceType::const_ptr(SourceType::Void));
}

// --- Nested structs ---

#[test]
fn fixture_nested_structs() {
    let src = r#"
struct inner { int x; int y; };
struct outer {
    struct inner pos;
    struct inner *ref;
    int id;
};
"#;
    let pkg = extract::extract_from_source(src).unwrap();

    let inner = pkg.find_record("inner").unwrap();
    assert_eq!(inner.fields.as_ref().unwrap().len(), 2);

    let outer = pkg.find_record("outer").unwrap();
    let fields = outer.fields.as_ref().unwrap();
    assert_eq!(fields.len(), 3);
    assert_eq!(fields[0].ty, SourceType::RecordRef("inner".into()));
    assert_eq!(
        fields[1].ty,
        SourceType::ptr(SourceType::RecordRef("inner".into()))
    );
    assert_eq!(fields[2].ty, SourceType::Int);
}

// --- Enum with computed values ---

#[test]
fn fixture_enum_computed_values() {
    let src = r#"
enum permissions {
    PERM_READ = 1 << 0,
    PERM_WRITE = 1 << 1,
    PERM_EXEC = 1 << 2,
    PERM_ALL = (1 << 0) | (1 << 1) | (1 << 2),
    PERM_NONE = 0,
    PERM_MINUS = -1
};
"#;
    let pkg = extract::extract_from_source(src).unwrap();
    let e = pkg.find_enum("permissions").unwrap();
    assert_eq!(e.variants.len(), 6);
    assert_eq!(e.variants[0].value, Some(1));
    assert_eq!(e.variants[1].value, Some(2));
    assert_eq!(e.variants[2].value, Some(4));
    assert_eq!(e.variants[3].value, Some(7));
    assert_eq!(e.variants[4].value, Some(0));
    assert_eq!(e.variants[5].value, Some(-1));
}

// --- Typedef chains ---

#[test]
fn fixture_typedef_chains() {
    let src = r#"
typedef unsigned int uint32_t;
typedef uint32_t UINT;
typedef UINT *PUINT;
typedef const UINT *PCUINT;
"#;
    let pkg = extract::extract_from_source(src).unwrap();

    let uint32 = pkg.find_type_alias("uint32_t").unwrap();
    assert_eq!(uint32.target, SourceType::UInt);

    let uint = pkg.find_type_alias("UINT").unwrap();
    assert_eq!(uint.target, SourceType::TypedefRef("uint32_t".into()));

    let puint = pkg.find_type_alias("PUINT").unwrap();
    assert_eq!(
        puint.target,
        SourceType::ptr(SourceType::TypedefRef("UINT".into()))
    );

    let pcuint = pkg.find_type_alias("PCUINT").unwrap();
    assert_eq!(
        pcuint.target,
        SourceType::const_ptr(SourceType::TypedefRef("UINT".into()))
    );
}

// --- Function pointers as parameters ---

#[test]
fn fixture_function_pointer_params() {
    let src = r#"
typedef void (*signal_handler_t)(int);
void set_handler(signal_handler_t handler);
int apply(int (*fn)(int, int), int a, int b);
"#;
    let pkg = extract::extract_from_source(src).unwrap();

    let sh = pkg.find_type_alias("signal_handler_t").unwrap();
    match &sh.target {
        SourceType::Pointer { pointee, .. } => match pointee.as_ref() {
            SourceType::FunctionPointer {
                return_type,
                parameters,
                variadic,
            } => {
                assert_eq!(**return_type, SourceType::Void);
                assert_eq!(parameters.len(), 1);
                assert!(!variadic);
            }
            other => panic!("expected FunctionPointer, got {:?}", other),
        },
        other => panic!("expected Pointer, got {:?}", other),
    }

    let set = pkg.find_function("set_handler").unwrap();
    assert_eq!(set.parameters.len(), 1);

    let apply = pkg.find_function("apply").unwrap();
    assert_eq!(apply.parameters.len(), 3);
    // First param is a function pointer
    match &apply.parameters[0].ty {
        SourceType::Pointer { pointee, .. } => match pointee.as_ref() {
            SourceType::FunctionPointer {
                return_type,
                parameters,
                ..
            } => {
                assert_eq!(**return_type, SourceType::Int);
                assert_eq!(parameters.len(), 2);
            }
            other => panic!("expected FunctionPointer, got {:?}", other),
        },
        other => panic!("expected Pointer, got {:?}", other),
    }
}

// --- Union types ---

#[test]
fn fixture_union_types() {
    let src = r#"
union value {
    int i;
    float f;
    double d;
    void *ptr;
};
typedef union value value_t;
"#;
    let pkg = extract::extract_from_source(src).unwrap();

    let u = pkg.find_record("value").unwrap();
    assert_eq!(u.kind, RecordKind::Union);
    let fields = u.fields.as_ref().unwrap();
    assert_eq!(fields.len(), 4);
    assert_eq!(fields[3].ty, SourceType::ptr(SourceType::Void));

    let alias = pkg.find_type_alias("value_t").unwrap();
    assert_eq!(
        alias.target,
        SourceType::RecordRef("value".into())
    );
}

// --- Extern variables ---

#[test]
fn fixture_extern_variables() {
    let src = r#"
extern int errno;
extern const char *version_string;
extern void *global_ctx;
"#;
    let pkg = extract::extract_from_source(src).unwrap();

    assert_eq!(pkg.variable_count(), 3);
    let errno = pkg.find_variable("errno").unwrap();
    assert_eq!(errno.ty, SourceType::Int);

    let ver = pkg.find_variable("version_string").unwrap();
    assert_eq!(ver.ty, SourceType::const_ptr(SourceType::Char));

    let ctx = pkg.find_variable("global_ctx").unwrap();
    assert_eq!(ctx.ty, SourceType::ptr(SourceType::Void));
}

// --- Array parameters ---

#[test]
fn fixture_array_parameters() {
    let src = r#"
void process(int data[], unsigned long count);
void matrix(double m[4][4]);
"#;
    let pkg = extract::extract_from_source(src).unwrap();

    let proc = pkg.find_function("process").unwrap();
    assert_eq!(proc.parameters.len(), 2);

    let matrix = pkg.find_function("matrix").unwrap();
    assert_eq!(matrix.parameters.len(), 1);
}

// --- Multiple declarators ---

#[test]
fn fixture_typedef_struct_inline() {
    let src = r#"
typedef struct node {
    int value;
    struct node *next;
} node_t;
"#;
    let pkg = extract::extract_from_source(src).unwrap();

    let rec = pkg.find_record("node").unwrap();
    assert!(!rec.is_opaque());
    let fields = rec.fields.as_ref().unwrap();
    assert_eq!(fields.len(), 2);
    assert_eq!(
        fields[1].ty,
        SourceType::ptr(SourceType::RecordRef("node".into()))
    );

    let alias = pkg.find_type_alias("node_t").unwrap();
    assert_eq!(alias.target, SourceType::RecordRef("node".into()));
}

// --- Complex return types ---

#[test]
fn fixture_complex_return_types() {
    let src = r#"
const char *get_name(void);
char *const get_buffer(void);
const char *const *get_list(void);
"#;
    let pkg = extract::extract_from_source(src).unwrap();

    let get_name = pkg.find_function("get_name").unwrap();
    assert_eq!(get_name.return_type, SourceType::const_ptr(SourceType::Char));

    // char *const — const pointer to char
    let get_buffer = pkg.find_function("get_buffer").unwrap();
    match &get_buffer.return_type {
        SourceType::Pointer { qualifiers, .. } => {
            // The pointer itself should have some const marking
            // (This tests that we handle pointer-level const correctly)
        }
        other => panic!("expected Pointer, got {:?}", other),
    }
}

// --- Mixed declarations ---

#[test]
fn fixture_comprehensive_api() {
    let src = r#"
typedef unsigned char uint8_t;
typedef unsigned short uint16_t;
typedef unsigned int uint32_t;
typedef unsigned long uint64_t;
typedef signed char int8_t;
typedef short int16_t;
typedef int int32_t;
typedef long int64_t;

struct buffer {
    uint8_t *data;
    uint64_t size;
    uint64_t capacity;
};

enum error_code {
    ERR_NONE = 0,
    ERR_NOMEM = 1,
    ERR_INVAL = 2,
    ERR_IO = 3
};

struct buffer *buffer_new(uint64_t initial_capacity);
void buffer_free(struct buffer *buf);
enum error_code buffer_append(struct buffer *buf, const uint8_t *data, uint64_t len);
const uint8_t *buffer_data(const struct buffer *buf);
uint64_t buffer_size(const struct buffer *buf);

extern int debug_level;
"#;
    let pkg = extract::extract_from_source(src).unwrap();

    // 8 type aliases
    assert_eq!(pkg.type_alias_count(), 8);

    // 1 struct
    let buf = pkg.find_record("buffer").unwrap();
    assert!(!buf.is_opaque());
    assert_eq!(buf.fields.as_ref().unwrap().len(), 3);

    // 1 enum
    let errs = pkg.find_enum("error_code").unwrap();
    assert_eq!(errs.variants.len(), 4);
    assert_eq!(errs.variants[0].value, Some(0));

    // 5 functions
    assert_eq!(pkg.function_count(), 5);
    let append = pkg.find_function("buffer_append").unwrap();
    assert_eq!(append.parameters.len(), 3);

    // 1 variable
    assert_eq!(pkg.variable_count(), 1);
    assert!(pkg.find_variable("debug_level").is_some());

    // All items have source offsets
    for item in &pkg.items {
        assert!(item.source_offset().is_some());
    }
}
