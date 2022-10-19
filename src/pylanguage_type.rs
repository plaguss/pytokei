use std::collections::hash_map::DefaultHasher;

// Required to call the `.hash` and `.finish` methods, which are defined on traits.
use std::hash::{Hash, Hasher};

use pyo3::class::basic::CompareOp;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use tokei::LanguageType;

// use crate::pyconfig::PyConfig;

fn language_type_mapper(lang_type: &str) -> Result<LanguageType, pyo3::PyErr> {
    match lang_type {
        "ABNF" => Ok(LanguageType::ABNF),
        "Abap" => Ok(LanguageType::Abap),
        "ActionScript" => Ok(LanguageType::ActionScript),
        "Ada" => Ok(LanguageType::Ada),
        "Agda" => Ok(LanguageType::Agda),
        "Alex" => Ok(LanguageType::Alex),
        "Alloy" => Ok(LanguageType::Alloy),
        "Arduino C++" => Ok(LanguageType::Arduino),
        "AsciiDoc" => Ok(LanguageType::AsciiDoc),
        "ASN.1" => Ok(LanguageType::Asn1),
        "ASP" => Ok(LanguageType::Asp),
        "ASP.NET" => Ok(LanguageType::AspNet),
        "Assembly" => Ok(LanguageType::Assembly),
        "GNU Style Assembly" => Ok(LanguageType::AssemblyGAS),
        "AutoHotKey" => Ok(LanguageType::AutoHotKey),
        "Autoconf" => Ok(LanguageType::Autoconf),
        "Automake" => Ok(LanguageType::Automake),
        "BASH" => Ok(LanguageType::Bash),
        "Batch" => Ok(LanguageType::Batch),
        "Bean" => Ok(LanguageType::Bean),
        "BrightScript" => Ok(LanguageType::BrightScript),
        "C" => Ok(LanguageType::C),
        "C Header" => Ok(LanguageType::CHeader),
        "CMake" => Ok(LanguageType::CMake),
        "C#" => Ok(LanguageType::CSharp),
        "C Shell" => Ok(LanguageType::CShell),
        "Cabal" => Ok(LanguageType::Cabal),
        "Cassius" => Ok(LanguageType::Cassius),
        "Ceylon" => Ok(LanguageType::Ceylon),
        "Clojure" => Ok(LanguageType::Clojure),
        "ClojureC" => Ok(LanguageType::ClojureC),
        "ClojureScript" => Ok(LanguageType::ClojureScript),
        "COBOL" => Ok(LanguageType::Cobol),
        "CodeQL" => Ok(LanguageType::CodeQL),
        "CoffeeScript" => Ok(LanguageType::CoffeeScript),
        "Cogent" => Ok(LanguageType::Cogent),
        "ColdFusion" => Ok(LanguageType::ColdFusion),
        "ColdFusion CFScript" => Ok(LanguageType::ColdFusionScript),
        "Coq" => Ok(LanguageType::Coq),
        "C++" => Ok(LanguageType::Cpp),
        "C++ Header" => Ok(LanguageType::CppHeader),
        "Crystal" => Ok(LanguageType::Crystal),
        "CSS" => Ok(LanguageType::Css),
        "D" => Ok(LanguageType::D),
        "DAML" => Ok(LanguageType::Daml),
        "Dart" => Ok(LanguageType::Dart),
        "Device Tree" => Ok(LanguageType::DeviceTree),
        "Dhall" => Ok(LanguageType::Dhall),
        "Dockerfile" => Ok(LanguageType::Dockerfile),
        ".NET Resource" => Ok(LanguageType::DotNetResource),
        "Dream Maker" => Ok(LanguageType::DreamMaker),
        "Dust.js" => Ok(LanguageType::Dust),
        "Edn" => Ok(LanguageType::Edn),
        "Emacs Lisp" => Ok(LanguageType::Elisp),
        "Elixir" => Ok(LanguageType::Elixir),
        "Elm" => Ok(LanguageType::Elm),
        "Elvish" => Ok(LanguageType::Elvish),
        "Emacs Dev Env" => Ok(LanguageType::EmacsDevEnv),
        "Emojicode" => Ok(LanguageType::Emojicode),
        "Erlang" => Ok(LanguageType::Erlang),
        "FEN" => Ok(LanguageType::FEN),
        "F#" => Ok(LanguageType::FSharp),
        "Fish" => Ok(LanguageType::Fish),
        "FlatBuffers Schema" => Ok(LanguageType::FlatBuffers),
        "Forth" => Ok(LanguageType::Forth),
        "FORTRAN Legacy" => Ok(LanguageType::FortranLegacy),
        "FORTRAN Modern" => Ok(LanguageType::FortranModern),
        "FreeMarker" => Ok(LanguageType::FreeMarker),
        "F*" => Ok(LanguageType::Fstar),
        "Futhark" => Ok(LanguageType::Futhark),
        "GDB Script" => Ok(LanguageType::GDB),
        "GDScript" => Ok(LanguageType::GdScript),
        "Gherkin (Cucumber)" => Ok(LanguageType::Gherkin),
        "Gleam" => Ok(LanguageType::Gleam),
        "GLSL" => Ok(LanguageType::Glsl),
        "Go" => Ok(LanguageType::Go),
        "Go HTML" => Ok(LanguageType::Gohtml),
        "GraphQL" => Ok(LanguageType::Graphql),
        "Groovy" => Ok(LanguageType::Groovy),
        "Gwion" => Ok(LanguageType::Gwion),
        "Hamlet" => Ok(LanguageType::Hamlet),
        "Handlebars" => Ok(LanguageType::Handlebars),
        "Happy" => Ok(LanguageType::Happy),
        "Haskell" => Ok(LanguageType::Haskell),
        "Haxe" => Ok(LanguageType::Haxe),
        "HCL" => Ok(LanguageType::Hcl),
        "Headache" => Ok(LanguageType::Headache),
        "HEX" => Ok(LanguageType::Hex),
        "HLSL" => Ok(LanguageType::Hlsl),
        "HolyC" => Ok(LanguageType::HolyC),
        "HTML" => Ok(LanguageType::Html),
        "Idris" => Ok(LanguageType::Idris),
        "INI" => Ok(LanguageType::Ini),
        "Intel HEX" => Ok(LanguageType::IntelHex),
        "Isabelle" => Ok(LanguageType::Isabelle),
        "JAI" => Ok(LanguageType::Jai),
        "Java" => Ok(LanguageType::Java),
        "JavaScript" => Ok(LanguageType::JavaScript),
        "JSON" => Ok(LanguageType::Json),
        "Jsonnet" => Ok(LanguageType::Jsonnet),
        "JSX" => Ok(LanguageType::Jsx),
        "Julia" => Ok(LanguageType::Julia),
        "Julius" => Ok(LanguageType::Julius),
        "Jupyter Notebooks" => Ok(LanguageType::Jupyter),
        "K" => Ok(LanguageType::K),
        "Kakoune script" => Ok(LanguageType::KakouneScript),
        "Kotlin" => Ok(LanguageType::Kotlin),
        "LLVM" => Ok(LanguageType::LLVM),
        "Lean" => Ok(LanguageType::Lean),
        "LESS" => Ok(LanguageType::Less),
        "LD Script" => Ok(LanguageType::LinkerScript),
        "Liquid" => Ok(LanguageType::Liquid),
        "Lisp" => Ok(LanguageType::Lisp),
        "LiveScript" => Ok(LanguageType::LiveScript),
        "Logtalk" => Ok(LanguageType::Logtalk),
        "Lua" => Ok(LanguageType::Lua),
        "Lucius" => Ok(LanguageType::Lucius),
        "Madlang" => Ok(LanguageType::Madlang),
        "Makefile" => Ok(LanguageType::Makefile),
        "Markdown" => Ok(LanguageType::Markdown),
        "Meson" => Ok(LanguageType::Meson),
        "Mint" => Ok(LanguageType::Mint),
        "Module-Definition" => Ok(LanguageType::ModuleDef),
        "MoonScript" => Ok(LanguageType::MoonScript),
        "MSBuild" => Ok(LanguageType::MsBuild),
        "Mustache" => Ok(LanguageType::Mustache),
        "Nim" => Ok(LanguageType::Nim),
        "Nix" => Ok(LanguageType::Nix),
        "Not Quite Perl" => Ok(LanguageType::NotQuitePerl),
        "OCaml" => Ok(LanguageType::OCaml),
        "Objective-C" => Ok(LanguageType::ObjectiveC),
        "Objective-C++" => Ok(LanguageType::ObjectiveCpp),
        "Odin" => Ok(LanguageType::Odin),
        "OpenType Feature File" => Ok(LanguageType::OpenType),
        "Org" => Ok(LanguageType::Org),
        "Oz" => Ok(LanguageType::Oz),
        "PSL Assertion" => Ok(LanguageType::PSL),
        "Pan" => Ok(LanguageType::Pan),
        "Pascal" => Ok(LanguageType::Pascal),
        "Perl" => Ok(LanguageType::Perl),
        "Rakudo" => Ok(LanguageType::Perl6),
        "Pest" => Ok(LanguageType::Pest),
        "PHP" => Ok(LanguageType::Php),
        "Polly" => Ok(LanguageType::Polly),
        "Pony" => Ok(LanguageType::Pony),
        "PostCSS" => Ok(LanguageType::PostCss),
        "PowerShell" => Ok(LanguageType::PowerShell),
        "Processing" => Ok(LanguageType::Processing),
        "Prolog" => Ok(LanguageType::Prolog),
        "Protocol Buffers" => Ok(LanguageType::Protobuf),
        "Pug" => Ok(LanguageType::Pug),
        "PureScript" => Ok(LanguageType::PureScript),
        "Python" => Ok(LanguageType::Python),
        "Q" => Ok(LanguageType::Q),
        "QCL" => Ok(LanguageType::Qcl),
        "QML" => Ok(LanguageType::Qml),
        "R" => Ok(LanguageType::R),
        "Rusty Object Notation" => Ok(LanguageType::RON),
        "RPM Specfile" => Ok(LanguageType::RPMSpecfile),
        "Racket" => Ok(LanguageType::Racket),
        "Rakefile" => Ok(LanguageType::Rakefile),
        "Razor" => Ok(LanguageType::Razor),
        "ReStructuredText" => Ok(LanguageType::ReStructuredText),
        "Ren'Py" => Ok(LanguageType::Renpy),
        "Ruby" => Ok(LanguageType::Ruby),
        "Ruby HTML" => Ok(LanguageType::RubyHtml),
        "Rust" => Ok(LanguageType::Rust),
        "SRecode Template" => Ok(LanguageType::SRecode),
        "Sass" => Ok(LanguageType::Sass),
        "Scala" => Ok(LanguageType::Scala),
        "Scheme" => Ok(LanguageType::Scheme),
        "Scons" => Ok(LanguageType::Scons),
        "Shell" => Ok(LanguageType::Sh),
        "Standard ML (SML)" => Ok(LanguageType::Sml),
        "Solidity" => Ok(LanguageType::Solidity),
        "Specman e" => Ok(LanguageType::SpecmanE),
        "Spice Netlist" => Ok(LanguageType::Spice),
        "SQL" => Ok(LanguageType::Sql),
        "Stan" => Ok(LanguageType::Stan),
        "Stratego/XT" => Ok(LanguageType::Stratego),
        "Stylus" => Ok(LanguageType::Stylus),
        "Svelte" => Ok(LanguageType::Svelte),
        "SVG" => Ok(LanguageType::Svg),
        "Swift" => Ok(LanguageType::Swift),
        "SWIG" => Ok(LanguageType::Swig),
        "SystemVerilog" => Ok(LanguageType::SystemVerilog),
        "TCL" => Ok(LanguageType::Tcl),
        "Tera" => Ok(LanguageType::Tera),
        "TeX" => Ok(LanguageType::Tex),
        "Plain Text" => Ok(LanguageType::Text),
        "Thrift" => Ok(LanguageType::Thrift),
        "TOML" => Ok(LanguageType::Toml),
        "TSX" => Ok(LanguageType::Tsx),
        "TTCN-3" => Ok(LanguageType::Ttcn),
        "Twig" => Ok(LanguageType::Twig),
        "TypeScript" => Ok(LanguageType::TypeScript),
        "Unreal Markdown" => Ok(LanguageType::UnrealDeveloperMarkdown),
        "Unreal Plugin" => Ok(LanguageType::UnrealPlugin),
        "Unreal Project" => Ok(LanguageType::UnrealProject),
        "Unreal Script" => Ok(LanguageType::UnrealScript),
        "Unreal Shader" => Ok(LanguageType::UnrealShader),
        "Unreal Shader Header" => Ok(LanguageType::UnrealShaderHeader),
        "Ur/Web" => Ok(LanguageType::UrWeb),
        "Ur/Web Project" => Ok(LanguageType::UrWebProject),
        "VB6" => Ok(LanguageType::VB6),
        "VBScript" => Ok(LanguageType::VBScript),
        "Vala" => Ok(LanguageType::Vala),
        "Apache Velocity" => Ok(LanguageType::Velocity),
        "Verilog" => Ok(LanguageType::Verilog),
        "Verilog Args File" => Ok(LanguageType::VerilogArgsFile),
        "VHDL" => Ok(LanguageType::Vhdl),
        "Vim script" => Ok(LanguageType::VimScript),
        "Visual Basic" => Ok(LanguageType::VisualBasic),
        "Visual Studio Project" => Ok(LanguageType::VisualStudioProject),
        "Visual Studio Solution" => Ok(LanguageType::VisualStudioSolution),
        "Vue" => Ok(LanguageType::Vue),
        "WebAssembly" => Ok(LanguageType::WebAssembly),
        "Wolfram" => Ok(LanguageType::Wolfram),
        "XSL" => Ok(LanguageType::XSL),
        "XAML" => Ok(LanguageType::Xaml),
        "Xcode Config" => Ok(LanguageType::XcodeConfig),
        "XML" => Ok(LanguageType::Xml),
        "Xtend" => Ok(LanguageType::Xtend),
        "YAML" => Ok(LanguageType::Yaml),
        "Zig" => Ok(LanguageType::Zig),
        "Zsh" => Ok(LanguageType::Zsh),
        _ => Err(PyValueError::new_err(format!(
            "LanguageType not found: {}",
            lang_type
        ))),
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[pyclass(name = "LanguageType")]
pub struct PyLanguageType(pub LanguageType);

#[pymethods]
impl PyLanguageType {
    #[new]
    pub fn new(lang_type_name: &str) -> PyResult<Self> {
        Ok(PyLanguageType(language_type_mapper(lang_type_name)?))
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

    pub fn name(&self) -> String {
        self.0.name().to_string()
    }

    #[staticmethod]
    pub fn list() -> Vec<&'static str> {
        let mut lang_types = Vec::new();
        for l in LanguageType::list() {
            lang_types.push(l.name());
        }
        lang_types
    }

    pub fn is_literate(&self) -> bool {
        self.0.is_literate()
    }

    pub fn line_comments(&self) -> Vec<&str> {
        let vec = Vec::from(self.0.line_comments());
        vec
    }

    pub fn multi_line_comments(&self) -> Vec<(&str, &str)> {
        let vec = Vec::from(self.0.multi_line_comments());
        vec
    }

    pub fn allows_nested(&self) -> bool {
        self.0.allows_nested()
    }

    pub fn nested_comments(&self) -> Vec<(&str, &str)> {
        let vec = Vec::from(self.0.nested_comments());
        vec
    }

    pub fn quotes(&self) -> Vec<(&str, &str)> {
        let vec = Vec::from(self.0.quotes());
        vec
    }

    pub fn verbatim_quotes(&self) -> Vec<(&str, &str)> {
        let vec = Vec::from(self.0.verbatim_quotes());
        vec
    }

    pub fn doc_quotes(&self) -> Vec<(&str, &str)> {
        let vec = Vec::from(self.0.doc_quotes());
        vec
    }

    pub fn shebangs(&self) -> Vec<&str> {
        let vec = Vec::from(self.0.shebangs());
        vec
    }

    pub fn important_syntax(&self) -> Vec<&str> {
        let vec = Vec::from(self.0.important_syntax());
        vec
    }

    /* TO BE DEVELOPED YET
    // #[staticmethod]
    // pub fn from_path(entry: &str, _config: &PyConfig) -> Option<Self> {
    // NOT DEVELOPED YET, WOULD NEED REWRITING THE new METHOD
    //     match LanguageType::from_path(entry, &_config.config) {
    //         Some(lang_type) => PyLanguageType()
    //     }
    // }

    pub fn from_file_extension(&self) -> bool {
        1
    }

    pub fn from_mime(&self) -> bool {
        1
    }

    pub fn from_shebang(&self) -> bool {
        1
    }
    */
}
