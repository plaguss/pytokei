
use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;

// Required to call the `.hash` and `.finish` methods, which are defined on traits.
use std::hash::{Hash, Hasher};

use pyo3::prelude::*;
use pyo3::class::basic::CompareOp;

use tokei::LanguageType;


// NOTE: Yet to decide how to deal with this
/*
fn lang_type_map(l: &str) -> LanguageType {
//fn lang_type_map() -> HashMap<&'static str, LanguageType> {
    let language_type_map = HashMap::from([
        ("ABNF", LanguageType::ABNF),
        ("ABAP", LanguageType::Abap),
    ]);
    language_type_map
}
*/

//NOTE: control possible inexistent language with Option, Some...
fn map_lang_type(lang_type: &str) -> LanguageType {
    match lang_type {
        "ABNF" => LanguageType::ABNF,
        "Abap" => LanguageType::Abap,
        "ActionScript" => LanguageType::ActionScript,
        "Ada" => LanguageType::Ada,
        "Agda" => LanguageType::Agda,
        "Alex" => LanguageType::Alex,
        "Alloy" => LanguageType::Alloy,
        "Arduino C++" => LanguageType::Arduino,
        "AsciiDoc" => LanguageType::AsciiDoc,
        "ASN.1" => LanguageType::Asn1,
        "ASP" => LanguageType::Asp,
        "ASP.NET" => LanguageType::AspNet,
        "Assembly" => LanguageType::Assembly,
        "GNU Style Assembly" => LanguageType::AssemblyGAS,
        "AutoHotKey" => LanguageType::AutoHotKey,
        "Autoconf" => LanguageType::Autoconf,
        "Automake" => LanguageType::Automake,
        "BASH" => LanguageType::Bash,
        "Batch" => LanguageType::Batch,
        "Bean" => LanguageType::Bean,
        "BrightScript" => LanguageType::BrightScript,
        "C" => LanguageType::C,
        "C Header" => LanguageType::CHeader,
        "CMake" => LanguageType::CMake,
        "C#" => LanguageType::CSharp,
        "C Shell" => LanguageType::CShell,
        "Cabal" => LanguageType::Cabal,
        "Cassius" => LanguageType::Cassius,
        "Ceylon" => LanguageType::Ceylon,
        "Clojure" => LanguageType::Clojure,
        "ClojureC" => LanguageType::ClojureC,
        "ClojureScript" => LanguageType::ClojureScript,
        "COBOL" => LanguageType::Cobol,
        "CodeQL" => LanguageType::CodeQL,
        "CoffeeScript" => LanguageType::CoffeeScript,
        "Cogent" => LanguageType::Cogent,
        "ColdFusion" => LanguageType::ColdFusion,
        "ColdFusion CFScript" => LanguageType::ColdFusionScript,
        "Coq" => LanguageType::Coq,
        "C++" => LanguageType::Cpp,
        "C++ Header" => LanguageType::CppHeader,
        "Crystal" => LanguageType::Crystal,
        "CSS" => LanguageType::Css,
        "D" => LanguageType::D,
        "DAML" => LanguageType::Daml,
        "Dart" => LanguageType::Dart,
        "Device Tree" => LanguageType::DeviceTree,
        "Dhall" => LanguageType::Dhall,
        "Dockerfile" => LanguageType::Dockerfile,
        ".NET Resource" => LanguageType::DotNetResource,
        "Dream Maker" => LanguageType::DreamMaker,
        "Dust.js" => LanguageType::Dust,
        "Edn" => LanguageType::Edn,
        "Emacs Lisp" => LanguageType::Elisp,
        "Elixir" => LanguageType::Elixir,
        "Elm" => LanguageType::Elm,
        "Elvish" => LanguageType::Elvish,
        "Emacs Dev Env" => LanguageType::EmacsDevEnv,
        "Emojicode" => LanguageType::Emojicode,
        "Erlang" => LanguageType::Erlang,
        "FEN" => LanguageType::FEN,
        "F#" => LanguageType::FSharp,
        "Fish" => LanguageType::Fish,
        "FlatBuffers Schema" => LanguageType::FlatBuffers,
        "Forth" => LanguageType::Forth,
        "FORTRAN Legacy" => LanguageType::FortranLegacy,
        "FORTRAN Modern" => LanguageType::FortranModern,
        "FreeMarker" => LanguageType::FreeMarker,
        "F*" => LanguageType::Fstar,
        "Futhark" => LanguageType::Futhark,
        "GDB Script" => LanguageType::GDB,
        "GDScript" => LanguageType::GdScript,
        "Gherkin (Cucumber)" => LanguageType::Gherkin,
        "Gleam" => LanguageType::Gleam,
        "GLSL" => LanguageType::Glsl,
        "Go" => LanguageType::Go,
        "Go HTML" => LanguageType::Gohtml,
        "GraphQL" => LanguageType::Graphql,
        "Groovy" => LanguageType::Groovy,
        "Gwion" => LanguageType::Gwion,
        "Hamlet" => LanguageType::Hamlet,
        "Handlebars" => LanguageType::Handlebars,
        "Happy" => LanguageType::Happy,
        "Haskell" => LanguageType::Haskell,
        "Haxe" => LanguageType::Haxe,
        "HCL" => LanguageType::Hcl,
        "Headache" => LanguageType::Headache,
        "HEX" => LanguageType::Hex,
        "HLSL" => LanguageType::Hlsl,
        "HolyC" => LanguageType::HolyC,
        "HTML" => LanguageType::Html,
        "Idris" => LanguageType::Idris,
        "INI" => LanguageType::Ini,
        "Intel HEX" => LanguageType::IntelHex,
        "Isabelle" => LanguageType::Isabelle,
        "JAI" => LanguageType::Jai,
        "Java" => LanguageType::Java,
        "JavaScript" => LanguageType::JavaScript,
        "JSON" => LanguageType::Json,
        "Jsonnet" => LanguageType::Jsonnet,
        "JSX" => LanguageType::Jsx,
        "Julia" => LanguageType::Julia,
        "Julius" => LanguageType::Julius,
        "Jupyter Notebooks" => LanguageType::Jupyter,
        "K" => LanguageType::K,
        "Kakoune script" => LanguageType::KakouneScript,
        "Kotlin" => LanguageType::Kotlin,
        "LLVM" => LanguageType::LLVM,
        "Lean" => LanguageType::Lean,
        "LESS" => LanguageType::Less,
        "LD Script" => LanguageType::LinkerScript,
        "Liquid" => LanguageType::Liquid,
        "Lisp" => LanguageType::Lisp,
        "LiveScript" => LanguageType::LiveScript,
        "Logtalk" => LanguageType::Logtalk,
        "Lua" => LanguageType::Lua,
        "Lucius" => LanguageType::Lucius,
        "Madlang" => LanguageType::Madlang,
        "Makefile" => LanguageType::Makefile,
        "Markdown" => LanguageType::Markdown,
        "Meson" => LanguageType::Meson,
        "Mint" => LanguageType::Mint,
        "Module-Definition" => LanguageType::ModuleDef,
        "MoonScript" => LanguageType::MoonScript,
        "MSBuild" => LanguageType::MsBuild,
        "Mustache" => LanguageType::Mustache,
        "Nim" => LanguageType::Nim,
        "Nix" => LanguageType::Nix,
        "Not Quite Perl" => LanguageType::NotQuitePerl,
        "OCaml" => LanguageType::OCaml,
        "Objective-C" => LanguageType::ObjectiveC,
        "Objective-C++" => LanguageType::ObjectiveCpp,
        "Odin" => LanguageType::Odin,
        "OpenType Feature File" => LanguageType::OpenType,
        "Org" => LanguageType::Org,
        "Oz" => LanguageType::Oz,
        "PSL Assertion" => LanguageType::PSL,
        "Pan" => LanguageType::Pan,
        "Pascal" => LanguageType::Pascal,
        "Perl" => LanguageType::Perl,
        "Rakudo" => LanguageType::Perl6,
        "Pest" => LanguageType::Pest,
        "PHP" => LanguageType::Php,
        "Polly" => LanguageType::Polly,
        "Pony" => LanguageType::Pony,
        "PostCSS" => LanguageType::PostCss,
        "PowerShell" => LanguageType::PowerShell,
        "Processing" => LanguageType::Processing,
        "Prolog" => LanguageType::Prolog,
        "Protocol Buffers" => LanguageType::Protobuf,
        "Pug" => LanguageType::Pug,
        "PureScript" => LanguageType::PureScript,
        "Python" => LanguageType::Python,
        "Q" => LanguageType::Q,
        "QCL" => LanguageType::Qcl,
        "QML" => LanguageType::Qml,
        "R" => LanguageType::R,
        "Rusty Object Notation" => LanguageType::RON,
        "RPM Specfile" => LanguageType::RPMSpecfile,
        "Racket" => LanguageType::Racket,
        "Rakefile" => LanguageType::Rakefile,
        "Razor" => LanguageType::Razor,
        "ReStructuredText" => LanguageType::ReStructuredText,
        "Ren'Py" => LanguageType::Renpy,
        "Ruby" => LanguageType::Ruby,
        "Ruby HTML" => LanguageType::RubyHtml,
        "Rust" => LanguageType::Rust,
        "SRecode Template" => LanguageType::SRecode,
        "Sass" => LanguageType::Sass,
        "Scala" => LanguageType::Scala,
        "Scheme" => LanguageType::Scheme,
        "Scons" => LanguageType::Scons,
        "Shell" => LanguageType::Sh,
        "Standard ML (SML)" => LanguageType::Sml,
        "Solidity" => LanguageType::Solidity,
        "Specman e" => LanguageType::SpecmanE,
        "Spice Netlist" => LanguageType::Spice,
        "SQL" => LanguageType::Sql,
        "Stan" => LanguageType::Stan,
        "Stratego/XT" => LanguageType::Stratego,
        "Stylus" => LanguageType::Stylus,
        "Svelte" => LanguageType::Svelte,
        "SVG" => LanguageType::Svg,
        "Swift" => LanguageType::Swift,
        "SWIG" => LanguageType::Swig,
        "SystemVerilog" => LanguageType::SystemVerilog,
        "TCL" => LanguageType::Tcl,
        "Tera" => LanguageType::Tera,
        "TeX" => LanguageType::Tex,
        "Plain Text" => LanguageType::Text,
        "Thrift" => LanguageType::Thrift,
        "TOML" => LanguageType::Toml,
        "TSX" => LanguageType::Tsx,
        "TTCN-3" => LanguageType::Ttcn,
        "Twig" => LanguageType::Twig,
        "TypeScript" => LanguageType::TypeScript,
        "Unreal Markdown" => LanguageType::UnrealDeveloperMarkdown,
        "Unreal Plugin" => LanguageType::UnrealPlugin,
        "Unreal Project" => LanguageType::UnrealProject,
        "Unreal Script" => LanguageType::UnrealScript,
        "Unreal Shader" => LanguageType::UnrealShader,
        "Unreal Shader Header" => LanguageType::UnrealShaderHeader,
        "Ur/Web" => LanguageType::UrWeb,
        "Ur/Web Project" => LanguageType::UrWebProject,
        "VB6" => LanguageType::VB6,
        "VBScript" => LanguageType::VBScript,
        "Vala" => LanguageType::Vala,
        "Apache Velocity" => LanguageType::Velocity,
        "Verilog" => LanguageType::Verilog,
        "Verilog Args File" => LanguageType::VerilogArgsFile,
        "VHDL" => LanguageType::Vhdl,
        "Vim script" => LanguageType::VimScript,
        "Visual Basic" => LanguageType::VisualBasic,
        "Visual Studio Project" => LanguageType::VisualStudioProject,
        "Visual Studio Solution" => LanguageType::VisualStudioSolution,
        "Vue" => LanguageType::Vue,
        "WebAssembly" => LanguageType::WebAssembly,
        "Wolfram" => LanguageType::Wolfram,
        "XSL" => LanguageType::XSL,
        "XAML" => LanguageType::Xaml,
        "Xcode Config" => LanguageType::XcodeConfig,
        "XML" => LanguageType::Xml,
        "Xtend" => LanguageType::Xtend,
        "YAML" => LanguageType::Yaml,
        "Zig" => LanguageType::Zig,
        "Zsh" => LanguageType::Zsh,
        // THIS IS BAD, YET TO DECIDE WHAT TO DO.
        &_ => LanguageType::ABNF,
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[pyclass(name="LanguageType")]
pub struct PyLanguageType(LanguageType);

#[pymethods]
impl PyLanguageType {
    #[new]
    pub fn new(lang_type_name: &str) -> PyResult<Self> {
        Ok(
            //            LanguageTypeContainer(lang_type_map(lang_type_name))
            PyLanguageType(map_lang_type(lang_type_name)),
        )
    }

    pub fn __hash__(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.0.hash(&mut hasher);
        hasher.finish()
    }

    pub fn __richcmp__(&self, other: &Self, op: CompareOp) -> PyResult<bool> {
        match op {
            CompareOp::Lt => Ok(self.0 < other.0),
            CompareOp::Le => Ok(self.0 <= other.0),
            CompareOp::Eq => Ok(self.0 == other.0),
            CompareOp::Ne => Ok(self.0 != other.0),
            CompareOp::Gt => Ok(self.0 > other.0),
            CompareOp::Ge => Ok(self.0 >= other.0),
        }
    }

    pub fn __repr__(&self) -> PyResult<String> {
        Ok(format!("LanguageType({:#?})", self.0))
    }
}

#[pyfunction]
pub fn language_types() -> HashMap<&'static str, PyLanguageType> {
    //pub fn language_types() -> HashMap<&'static str, &'static str> {
    //pub fn language_types() -> HashMap<String, String> {
    let mut lang_types = HashMap::new();

    for l in LanguageType::list() {
        lang_types.insert(l.name(), PyLanguageType(map_lang_type(l.name())));
        //        lang_types.insert(l.name(), l.name());
    }

    lang_types
}
