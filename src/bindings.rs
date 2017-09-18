/* automatically generated by rust-bindgen */

pub type khronos_uint64_t = u64;
pub type std_array_value_type = u8;
pub type std_array_pointer = u8;
pub type std_array_const_pointer = u8;
pub type std_array_reference = u8;
pub type std_array_const_reference = u8;
pub type std_array_iterator = u8;
pub type std_array_const_iterator = u8;
pub type std_array_size_type = u64;
pub type std_array_difference_type = u64;
pub type std_array_reverse_iterator = u8;
pub type std_array_const_reverse_iterator = u8;
pub type std_array__AT_Type = u8;
pub type ShCompileOptions = u64;

pub const SH_VALIDATE : ShCompileOptions = 0;
pub const SH_VALIDATE_LOOP_INDEXING : ShCompileOptions = 1u64 << 0;
pub const SH_INTERMEDIATE_TREE : ShCompileOptions = 1u64 << 1;
pub const SH_OBJECT_CODE : ShCompileOptions = 1u64 << 2;
pub const SH_VARIABLES : ShCompileOptions = 1u64 << 3;
pub const SH_LINE_DIRECTIVES : ShCompileOptions = 1u64 << 4;
pub const SH_SOURCE_PATH : ShCompileOptions = 1u64 << 5;
pub const SH_DONT_REMOVE_INVARIANT_FOR_FRAGMENT_INPUT : ShCompileOptions = 1u64 << 6;
pub const SH_REMOVE_INVARIANT_AND_CENTROID_FOR_ESSL3 : ShCompileOptions = 1u64 << 7;
pub const SH_EMULATE_ABS_INT_FUNCTION : ShCompileOptions = 1u64 << 8;
pub const SH_ENFORCE_PACKING_RESTRICTIONS : ShCompileOptions = 1u64 << 9;
pub const SH_CLAMP_INDIRECT_ARRAY_BOUNDS : ShCompileOptions = 1u64 << 10;
pub const SH_LIMIT_EXPRESSION_COMPLEXITY : ShCompileOptions = 1u64 << 11;
pub const SH_LIMIT_CALL_STACK_DEPTH : ShCompileOptions = 1u64 << 12;
pub const SH_INIT_GL_POSITION : ShCompileOptions = 1u64 << 13;
pub const SH_UNFOLD_SHORT_CIRCUIT : ShCompileOptions = 1u64 << 14;
pub const SH_INIT_OUTPUT_VARIABLES : ShCompileOptions = 1u64 << 15;
pub const SH_SCALARIZE_VEC_AND_MAT_CONSTRUCTOR_ARGS : ShCompileOptions = 1u64 << 16;
pub const SH_REGENERATE_STRUCT_NAMES : ShCompileOptions = 1u64 << 17;
pub const SH_DONT_PRUNE_UNUSED_FUNCTIONS : ShCompileOptions = 1u64 << 18;
pub const SH_REMOVE_POW_WITH_CONSTANT_EXPONENT : ShCompileOptions = 1u64 << 19;
pub const SH_REWRITE_DO_WHILE_LOOPS : ShCompileOptions = 1u64 << 20;
pub const SH_EXPAND_SELECT_HLSL_INTEGER_POW_EXPRESSIONS : ShCompileOptions = 1u64 << 21;
pub const SH_FLATTEN_PRAGMA_STDGL_INVARIANT_ALL : ShCompileOptions = 1u64 << 22;
pub const SH_HLSL_GET_DIMENSIONS_IGNORES_BASE_LEVEL : ShCompileOptions = 1u64 << 23;
pub const SH_REWRITE_TEXELFETCHOFFSET_TO_TEXELFETCH : ShCompileOptions = 1u64 << 24;
pub const SH_ADD_AND_TRUE_TO_LOOP_CONDITION : ShCompileOptions = 1u64 << 25;
pub const SH_REWRITE_INTEGER_UNARY_MINUS_OPERATOR : ShCompileOptions = 1u64 << 26;
pub const SH_EMULATE_ISNAN_FLOAT_FUNCTION : ShCompileOptions = 1u64 << 27;
pub const SH_USE_UNUSED_STANDARD_SHARED_BLOCKS : ShCompileOptions = 1u64 << 28;
pub const SH_REWRITE_FLOAT_UNARY_MINUS_OPERATOR : ShCompileOptions = 1u64 << 29;
pub const SH_EMULATE_ATAN2_FLOAT_FUNCTION : ShCompileOptions = 1u64 << 30;
pub const SH_TRANSLATE_VIEWID_OVR_TO_UNIFORM : ShCompileOptions = 1u64 << 31;
pub const SH_INITIALIZE_UNINITIALIZED_LOCALS : ShCompileOptions = 1u64 << 32;
pub const SH_INITIALIZE_BUILTINS_FOR_INSTANCED_MULTIVIEW : ShCompileOptions = 1u64 << 33;
pub const SH_SELECT_VIEW_IN_NV_GLSL_VERTEX_SHADER : ShCompileOptions = 1u64 << 34;
pub const SH_CLAMP_POINT_SIZE : ShCompileOptions = 1u64 << 35;

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ShShaderSpec {
    SH_GLES2_SPEC = 0,
    SH_WEBGL_SPEC = 1,
    SH_GLES3_SPEC = 2,
    SH_WEBGL2_SPEC = 3,
    SH_GLES3_1_SPEC = 4,
    SH_WEBGL3_SPEC = 5,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ShShaderOutput {
    SH_ESSL_OUTPUT = 35653,
    SH_GLSL_COMPATIBILITY_OUTPUT = 35654,
    SH_GLSL_130_OUTPUT = 35655,
    SH_GLSL_140_OUTPUT = 35712,
    SH_GLSL_150_CORE_OUTPUT = 35713,
    SH_GLSL_330_CORE_OUTPUT = 35714,
    SH_GLSL_400_CORE_OUTPUT = 35715,
    SH_GLSL_410_CORE_OUTPUT = 35716,
    SH_GLSL_420_CORE_OUTPUT = 35717,
    SH_GLSL_430_CORE_OUTPUT = 35718,
    SH_GLSL_440_CORE_OUTPUT = 35719,
    SH_GLSL_450_CORE_OUTPUT = 35720,
    SH_HLSL_3_0_OUTPUT = 35656,
    SH_HLSL_4_1_OUTPUT = 35657,
    SH_HLSL_4_0_FL9_3_OUTPUT = 35658,
    SH_GLSL_VULKAN_OUTPUT = 35659,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ShArrayIndexClampingStrategy {
    SH_CLAMP_WITH_CLAMP_INTRINSIC = 1,
    SH_CLAMP_WITH_USER_DEFINED_INT_CLAMP_FUNCTION = 2,
}
pub type ShHashFunction64 =
    ::std::option::Option<unsafe extern "C" fn(arg1:
                                                   *const ::std::os::raw::c_char,
                                               arg2: usize)
                              -> khronos_uint64_t>;
#[repr(C)]
#[derive(Debug, Copy)]
pub struct ShBuiltInResources {
    pub MaxVertexAttribs: ::std::os::raw::c_int,
    pub MaxVertexUniformVectors: ::std::os::raw::c_int,
    pub MaxVaryingVectors: ::std::os::raw::c_int,
    pub MaxVertexTextureImageUnits: ::std::os::raw::c_int,
    pub MaxCombinedTextureImageUnits: ::std::os::raw::c_int,
    pub MaxTextureImageUnits: ::std::os::raw::c_int,
    pub MaxFragmentUniformVectors: ::std::os::raw::c_int,
    pub MaxDrawBuffers: ::std::os::raw::c_int,
    pub OES_standard_derivatives: ::std::os::raw::c_int,
    pub OES_EGL_image_external: ::std::os::raw::c_int,
    pub OES_EGL_image_external_essl3: ::std::os::raw::c_int,
    pub NV_EGL_stream_consumer_external: ::std::os::raw::c_int,
    pub ARB_texture_rectangle: ::std::os::raw::c_int,
    pub EXT_blend_func_extended: ::std::os::raw::c_int,
    pub EXT_draw_buffers: ::std::os::raw::c_int,
    pub EXT_frag_depth: ::std::os::raw::c_int,
    pub EXT_shader_texture_lod: ::std::os::raw::c_int,
    pub WEBGL_debug_shader_precision: ::std::os::raw::c_int,
    pub EXT_shader_framebuffer_fetch: ::std::os::raw::c_int,
    pub NV_shader_framebuffer_fetch: ::std::os::raw::c_int,
    pub ARM_shader_framebuffer_fetch: ::std::os::raw::c_int,
    pub OVR_multiview: ::std::os::raw::c_int,
    pub EXT_YUV_target: ::std::os::raw::c_int,
    pub OES_geometry_shader: ::std::os::raw::c_int,
    pub NV_draw_buffers: ::std::os::raw::c_int,
    pub FragmentPrecisionHigh: ::std::os::raw::c_int,
    pub MaxVertexOutputVectors: ::std::os::raw::c_int,
    pub MaxFragmentInputVectors: ::std::os::raw::c_int,
    pub MinProgramTexelOffset: ::std::os::raw::c_int,
    pub MaxProgramTexelOffset: ::std::os::raw::c_int,
    pub MaxDualSourceDrawBuffers: ::std::os::raw::c_int,
    pub MaxViewsOVR: ::std::os::raw::c_int,
    pub HashFunction: ShHashFunction64,
    pub ArrayIndexClampingStrategy: ShArrayIndexClampingStrategy,
    pub MaxExpressionComplexity: ::std::os::raw::c_int,
    pub MaxCallStackDepth: ::std::os::raw::c_int,
    pub MaxFunctionParameters: ::std::os::raw::c_int,
    pub MaxImageUnits: ::std::os::raw::c_int,
    pub MaxVertexImageUniforms: ::std::os::raw::c_int,
    pub MaxFragmentImageUniforms: ::std::os::raw::c_int,
    pub MaxComputeImageUniforms: ::std::os::raw::c_int,
    pub MaxCombinedImageUniforms: ::std::os::raw::c_int,
    pub MaxUniformLocations: ::std::os::raw::c_int,
    pub MaxCombinedShaderOutputResources: ::std::os::raw::c_int,
    pub MaxComputeWorkGroupCount: [u32; 3usize],
    pub MaxComputeWorkGroupSize: [u32; 3usize],
    pub MaxComputeUniformComponents: ::std::os::raw::c_int,
    pub MaxComputeTextureImageUnits: ::std::os::raw::c_int,
    pub MaxComputeAtomicCounters: ::std::os::raw::c_int,
    pub MaxComputeAtomicCounterBuffers: ::std::os::raw::c_int,
    pub MaxVertexAtomicCounters: ::std::os::raw::c_int,
    pub MaxFragmentAtomicCounters: ::std::os::raw::c_int,
    pub MaxCombinedAtomicCounters: ::std::os::raw::c_int,
    pub MaxAtomicCounterBindings: ::std::os::raw::c_int,
    pub MaxVertexAtomicCounterBuffers: ::std::os::raw::c_int,
    pub MaxFragmentAtomicCounterBuffers: ::std::os::raw::c_int,
    pub MaxCombinedAtomicCounterBuffers: ::std::os::raw::c_int,
    pub MaxAtomicCounterBufferSize: ::std::os::raw::c_int,
    pub MaxUniformBufferBindings: ::std::os::raw::c_int,
    pub MaxShaderStorageBufferBindings: ::std::os::raw::c_int,
    pub MaxPointSize: f32,
    pub MaxGeometryUniformComponents: ::std::os::raw::c_int,
    pub MaxGeometryOutputVertices: ::std::os::raw::c_int,
    pub MaxGeometryShaderInvocations: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_ShBuiltInResources() {
    assert_eq!(::std::mem::size_of::<ShBuiltInResources>() , 280usize , concat
               ! ( "Size of: " , stringify ! ( ShBuiltInResources ) ));
    assert_eq! (::std::mem::align_of::<ShBuiltInResources>() , 8usize , concat
                ! ( "Alignment of " , stringify ! ( ShBuiltInResources ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ShBuiltInResources ) ) . MaxVertexAttribs
                as * const _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( ShBuiltInResources ) ,
                "::" , stringify ! ( MaxVertexAttribs ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ShBuiltInResources ) ) .
                MaxVertexUniformVectors as * const _ as usize } , 4usize ,
                concat ! (
                "Alignment of field: " , stringify ! ( ShBuiltInResources ) ,
                "::" , stringify ! ( MaxVertexUniformVectors ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ShBuiltInResources ) ) .
                MaxVaryingVectors as * const _ as usize } , 8usize , concat !
                (
                "Alignment of field: " , stringify ! ( ShBuiltInResources ) ,
                "::" , stringify ! ( MaxVaryingVectors ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ShBuiltInResources ) ) .
                MaxVertexTextureImageUnits as * const _ as usize } , 12usize ,
                concat ! (
                "Alignment of field: " , stringify ! ( ShBuiltInResources ) ,
                "::" , stringify ! ( MaxVertexTextureImageUnits ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ShBuiltInResources ) ) .
                MaxCombinedTextureImageUnits as * const _ as usize } , 16usize
                , concat ! (
                "Alignment of field: " , stringify ! ( ShBuiltInResources ) ,
                "::" , stringify ! ( MaxCombinedTextureImageUnits ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ShBuiltInResources ) ) .
                MaxTextureImageUnits as * const _ as usize } , 20usize ,
                concat ! (
                "Alignment of field: " , stringify ! ( ShBuiltInResources ) ,
                "::" , stringify ! ( MaxTextureImageUnits ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ShBuiltInResources ) ) .
                MaxFragmentUniformVectors as * const _ as usize } , 24usize ,
                concat ! (
                "Alignment of field: " , stringify ! ( ShBuiltInResources ) ,
                "::" , stringify ! ( MaxFragmentUniformVectors ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ShBuiltInResources ) ) . MaxDrawBuffers
                as * const _ as usize } , 28usize , concat ! (
                "Alignment of field: " , stringify ! ( ShBuiltInResources ) ,
                "::" , stringify ! ( MaxDrawBuffers ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ShBuiltInResources ) ) .
                OES_standard_derivatives as * const _ as usize } , 32usize ,
                concat ! (
                "Alignment of field: " , stringify ! ( ShBuiltInResources ) ,
                "::" , stringify ! ( OES_standard_derivatives ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ShBuiltInResources ) ) .
                OES_EGL_image_external as * const _ as usize } , 36usize ,
                concat ! (
                "Alignment of field: " , stringify ! ( ShBuiltInResources ) ,
                "::" , stringify ! ( OES_EGL_image_external ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ShBuiltInResources ) ) .
                OES_EGL_image_external_essl3 as * const _ as usize } , 40usize
                , concat ! (
                "Alignment of field: " , stringify ! ( ShBuiltInResources ) ,
                "::" , stringify ! ( OES_EGL_image_external_essl3 ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ShBuiltInResources ) ) .
                NV_EGL_stream_consumer_external as * const _ as usize } ,
                44usize , concat ! (
                "Alignment of field: " , stringify ! ( ShBuiltInResources ) ,
                "::" , stringify ! ( NV_EGL_stream_consumer_external ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ShBuiltInResources ) ) .
                ARB_texture_rectangle as * const _ as usize } , 48usize ,
                concat ! (
                "Alignment of field: " , stringify ! ( ShBuiltInResources ) ,
                "::" , stringify ! ( ARB_texture_rectangle ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ShBuiltInResources ) ) .
                EXT_blend_func_extended as * const _ as usize } , 52usize ,
                concat ! (
                "Alignment of field: " , stringify ! ( ShBuiltInResources ) ,
                "::" , stringify ! ( EXT_blend_func_extended ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ShBuiltInResources ) ) . EXT_draw_buffers
                as * const _ as usize } , 56usize , concat ! (
                "Alignment of field: " , stringify ! ( ShBuiltInResources ) ,
                "::" , stringify ! ( EXT_draw_buffers ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ShBuiltInResources ) ) . EXT_frag_depth
                as * const _ as usize } , 60usize , concat ! (
                "Alignment of field: " , stringify ! ( ShBuiltInResources ) ,
                "::" , stringify ! ( EXT_frag_depth ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ShBuiltInResources ) ) .
                EXT_shader_texture_lod as * const _ as usize } , 64usize ,
                concat ! (
                "Alignment of field: " , stringify ! ( ShBuiltInResources ) ,
                "::" , stringify ! ( EXT_shader_texture_lod ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ShBuiltInResources ) ) .
                WEBGL_debug_shader_precision as * const _ as usize } , 68usize
                , concat ! (
                "Alignment of field: " , stringify ! ( ShBuiltInResources ) ,
                "::" , stringify ! ( WEBGL_debug_shader_precision ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ShBuiltInResources ) ) .
                EXT_shader_framebuffer_fetch as * const _ as usize } , 72usize
                , concat ! (
                "Alignment of field: " , stringify ! ( ShBuiltInResources ) ,
                "::" , stringify ! ( EXT_shader_framebuffer_fetch ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ShBuiltInResources ) ) .
                NV_shader_framebuffer_fetch as * const _ as usize } , 76usize
                , concat ! (
                "Alignment of field: " , stringify ! ( ShBuiltInResources ) ,
                "::" , stringify ! ( NV_shader_framebuffer_fetch ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ShBuiltInResources ) ) .
                ARM_shader_framebuffer_fetch as * const _ as usize } , 80usize
                , concat ! (
                "Alignment of field: " , stringify ! ( ShBuiltInResources ) ,
                "::" , stringify ! ( ARM_shader_framebuffer_fetch ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ShBuiltInResources ) ) . OVR_multiview as
                * const _ as usize } , 84usize , concat ! (
                "Alignment of field: " , stringify ! ( ShBuiltInResources ) ,
                "::" , stringify ! ( OVR_multiview ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ShBuiltInResources ) ) . EXT_YUV_target
                as * const _ as usize } , 88usize , concat ! (
                "Alignment of field: " , stringify ! ( ShBuiltInResources ) ,
                "::" , stringify ! ( EXT_YUV_target ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ShBuiltInResources ) ) .
                OES_geometry_shader as * const _ as usize } , 92usize , concat
                ! (
                "Alignment of field: " , stringify ! ( ShBuiltInResources ) ,
                "::" , stringify ! ( OES_geometry_shader ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ShBuiltInResources ) ) . NV_draw_buffers
                as * const _ as usize } , 96usize , concat ! (
                "Alignment of field: " , stringify ! ( ShBuiltInResources ) ,
                "::" , stringify ! ( NV_draw_buffers ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ShBuiltInResources ) ) .
                FragmentPrecisionHigh as * const _ as usize } , 100usize ,
                concat ! (
                "Alignment of field: " , stringify ! ( ShBuiltInResources ) ,
                "::" , stringify ! ( FragmentPrecisionHigh ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ShBuiltInResources ) ) .
                MaxVertexOutputVectors as * const _ as usize } , 104usize ,
                concat ! (
                "Alignment of field: " , stringify ! ( ShBuiltInResources ) ,
                "::" , stringify ! ( MaxVertexOutputVectors ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ShBuiltInResources ) ) .
                MaxFragmentInputVectors as * const _ as usize } , 108usize ,
                concat ! (
                "Alignment of field: " , stringify ! ( ShBuiltInResources ) ,
                "::" , stringify ! ( MaxFragmentInputVectors ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ShBuiltInResources ) ) .
                MinProgramTexelOffset as * const _ as usize } , 112usize ,
                concat ! (
                "Alignment of field: " , stringify ! ( ShBuiltInResources ) ,
                "::" , stringify ! ( MinProgramTexelOffset ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ShBuiltInResources ) ) .
                MaxProgramTexelOffset as * const _ as usize } , 116usize ,
                concat ! (
                "Alignment of field: " , stringify ! ( ShBuiltInResources ) ,
                "::" , stringify ! ( MaxProgramTexelOffset ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ShBuiltInResources ) ) .
                MaxDualSourceDrawBuffers as * const _ as usize } , 120usize ,
                concat ! (
                "Alignment of field: " , stringify ! ( ShBuiltInResources ) ,
                "::" , stringify ! ( MaxDualSourceDrawBuffers ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ShBuiltInResources ) ) . MaxViewsOVR as *
                const _ as usize } , 124usize , concat ! (
                "Alignment of field: " , stringify ! ( ShBuiltInResources ) ,
                "::" , stringify ! ( MaxViewsOVR ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ShBuiltInResources ) ) . HashFunction as
                * const _ as usize } , 128usize , concat ! (
                "Alignment of field: " , stringify ! ( ShBuiltInResources ) ,
                "::" , stringify ! ( HashFunction ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ShBuiltInResources ) ) .
                ArrayIndexClampingStrategy as * const _ as usize } , 136usize
                , concat ! (
                "Alignment of field: " , stringify ! ( ShBuiltInResources ) ,
                "::" , stringify ! ( ArrayIndexClampingStrategy ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ShBuiltInResources ) ) .
                MaxExpressionComplexity as * const _ as usize } , 140usize ,
                concat ! (
                "Alignment of field: " , stringify ! ( ShBuiltInResources ) ,
                "::" , stringify ! ( MaxExpressionComplexity ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ShBuiltInResources ) ) .
                MaxCallStackDepth as * const _ as usize } , 144usize , concat
                ! (
                "Alignment of field: " , stringify ! ( ShBuiltInResources ) ,
                "::" , stringify ! ( MaxCallStackDepth ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ShBuiltInResources ) ) .
                MaxFunctionParameters as * const _ as usize } , 148usize ,
                concat ! (
                "Alignment of field: " , stringify ! ( ShBuiltInResources ) ,
                "::" , stringify ! ( MaxFunctionParameters ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ShBuiltInResources ) ) . MaxImageUnits as
                * const _ as usize } , 152usize , concat ! (
                "Alignment of field: " , stringify ! ( ShBuiltInResources ) ,
                "::" , stringify ! ( MaxImageUnits ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ShBuiltInResources ) ) .
                MaxVertexImageUniforms as * const _ as usize } , 156usize ,
                concat ! (
                "Alignment of field: " , stringify ! ( ShBuiltInResources ) ,
                "::" , stringify ! ( MaxVertexImageUniforms ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ShBuiltInResources ) ) .
                MaxFragmentImageUniforms as * const _ as usize } , 160usize ,
                concat ! (
                "Alignment of field: " , stringify ! ( ShBuiltInResources ) ,
                "::" , stringify ! ( MaxFragmentImageUniforms ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ShBuiltInResources ) ) .
                MaxComputeImageUniforms as * const _ as usize } , 164usize ,
                concat ! (
                "Alignment of field: " , stringify ! ( ShBuiltInResources ) ,
                "::" , stringify ! ( MaxComputeImageUniforms ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ShBuiltInResources ) ) .
                MaxCombinedImageUniforms as * const _ as usize } , 168usize ,
                concat ! (
                "Alignment of field: " , stringify ! ( ShBuiltInResources ) ,
                "::" , stringify ! ( MaxCombinedImageUniforms ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ShBuiltInResources ) ) .
                MaxUniformLocations as * const _ as usize } , 172usize ,
                concat ! (
                "Alignment of field: " , stringify ! ( ShBuiltInResources ) ,
                "::" , stringify ! ( MaxUniformLocations ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ShBuiltInResources ) ) .
                MaxCombinedShaderOutputResources as * const _ as usize } ,
                176usize , concat ! (
                "Alignment of field: " , stringify ! ( ShBuiltInResources ) ,
                "::" , stringify ! ( MaxCombinedShaderOutputResources ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ShBuiltInResources ) ) .
                MaxComputeWorkGroupCount as * const _ as usize } , 180usize ,
                concat ! (
                "Alignment of field: " , stringify ! ( ShBuiltInResources ) ,
                "::" , stringify ! ( MaxComputeWorkGroupCount ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ShBuiltInResources ) ) .
                MaxComputeWorkGroupSize as * const _ as usize } , 192usize ,
                concat ! (
                "Alignment of field: " , stringify ! ( ShBuiltInResources ) ,
                "::" , stringify ! ( MaxComputeWorkGroupSize ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ShBuiltInResources ) ) .
                MaxComputeUniformComponents as * const _ as usize } , 204usize
                , concat ! (
                "Alignment of field: " , stringify ! ( ShBuiltInResources ) ,
                "::" , stringify ! ( MaxComputeUniformComponents ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ShBuiltInResources ) ) .
                MaxComputeTextureImageUnits as * const _ as usize } , 208usize
                , concat ! (
                "Alignment of field: " , stringify ! ( ShBuiltInResources ) ,
                "::" , stringify ! ( MaxComputeTextureImageUnits ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ShBuiltInResources ) ) .
                MaxComputeAtomicCounters as * const _ as usize } , 212usize ,
                concat ! (
                "Alignment of field: " , stringify ! ( ShBuiltInResources ) ,
                "::" , stringify ! ( MaxComputeAtomicCounters ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ShBuiltInResources ) ) .
                MaxComputeAtomicCounterBuffers as * const _ as usize } ,
                216usize , concat ! (
                "Alignment of field: " , stringify ! ( ShBuiltInResources ) ,
                "::" , stringify ! ( MaxComputeAtomicCounterBuffers ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ShBuiltInResources ) ) .
                MaxVertexAtomicCounters as * const _ as usize } , 220usize ,
                concat ! (
                "Alignment of field: " , stringify ! ( ShBuiltInResources ) ,
                "::" , stringify ! ( MaxVertexAtomicCounters ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ShBuiltInResources ) ) .
                MaxFragmentAtomicCounters as * const _ as usize } , 224usize ,
                concat ! (
                "Alignment of field: " , stringify ! ( ShBuiltInResources ) ,
                "::" , stringify ! ( MaxFragmentAtomicCounters ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ShBuiltInResources ) ) .
                MaxCombinedAtomicCounters as * const _ as usize } , 228usize ,
                concat ! (
                "Alignment of field: " , stringify ! ( ShBuiltInResources ) ,
                "::" , stringify ! ( MaxCombinedAtomicCounters ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ShBuiltInResources ) ) .
                MaxAtomicCounterBindings as * const _ as usize } , 232usize ,
                concat ! (
                "Alignment of field: " , stringify ! ( ShBuiltInResources ) ,
                "::" , stringify ! ( MaxAtomicCounterBindings ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ShBuiltInResources ) ) .
                MaxVertexAtomicCounterBuffers as * const _ as usize } ,
                236usize , concat ! (
                "Alignment of field: " , stringify ! ( ShBuiltInResources ) ,
                "::" , stringify ! ( MaxVertexAtomicCounterBuffers ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ShBuiltInResources ) ) .
                MaxFragmentAtomicCounterBuffers as * const _ as usize } ,
                240usize , concat ! (
                "Alignment of field: " , stringify ! ( ShBuiltInResources ) ,
                "::" , stringify ! ( MaxFragmentAtomicCounterBuffers ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ShBuiltInResources ) ) .
                MaxCombinedAtomicCounterBuffers as * const _ as usize } ,
                244usize , concat ! (
                "Alignment of field: " , stringify ! ( ShBuiltInResources ) ,
                "::" , stringify ! ( MaxCombinedAtomicCounterBuffers ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ShBuiltInResources ) ) .
                MaxAtomicCounterBufferSize as * const _ as usize } , 248usize
                , concat ! (
                "Alignment of field: " , stringify ! ( ShBuiltInResources ) ,
                "::" , stringify ! ( MaxAtomicCounterBufferSize ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ShBuiltInResources ) ) .
                MaxUniformBufferBindings as * const _ as usize } , 252usize ,
                concat ! (
                "Alignment of field: " , stringify ! ( ShBuiltInResources ) ,
                "::" , stringify ! ( MaxUniformBufferBindings ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ShBuiltInResources ) ) .
                MaxShaderStorageBufferBindings as * const _ as usize } ,
                256usize , concat ! (
                "Alignment of field: " , stringify ! ( ShBuiltInResources ) ,
                "::" , stringify ! ( MaxShaderStorageBufferBindings ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ShBuiltInResources ) ) . MaxPointSize as
                * const _ as usize } , 260usize , concat ! (
                "Alignment of field: " , stringify ! ( ShBuiltInResources ) ,
                "::" , stringify ! ( MaxPointSize ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ShBuiltInResources ) ) .
                MaxGeometryUniformComponents as * const _ as usize } ,
                264usize , concat ! (
                "Alignment of field: " , stringify ! ( ShBuiltInResources ) ,
                "::" , stringify ! ( MaxGeometryUniformComponents ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ShBuiltInResources ) ) .
                MaxGeometryOutputVertices as * const _ as usize } , 268usize ,
                concat ! (
                "Alignment of field: " , stringify ! ( ShBuiltInResources ) ,
                "::" , stringify ! ( MaxGeometryOutputVertices ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const ShBuiltInResources ) ) .
                MaxGeometryShaderInvocations as * const _ as usize } ,
                272usize , concat ! (
                "Alignment of field: " , stringify ! ( ShBuiltInResources ) ,
                "::" , stringify ! ( MaxGeometryShaderInvocations ) ));
}
impl Clone for ShBuiltInResources {
    fn clone(&self) -> Self { *self }
}
pub type ShHandle = *mut ::std::os::raw::c_void;