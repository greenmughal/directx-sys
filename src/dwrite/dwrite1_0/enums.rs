use winapi::UINT;

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MeasuringMode {
    Natural,
    GDIClassic,
    GDINatural
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FontFileType {
    Unknown,
    CFF,
    TrueType,
    TrueTypeCollection,
    Type1PFM,
    Type1PFB,
    Vector,
    Bitmap
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FontFaceType {
    CFF,
    TrueType,
    TrueTypeCollection,
    Type1,
    Vector,
    Bitmap,
    Unknown,
    RawCFF
}

bitflags! {
    #[repr(C)]
    flags FontSimulations: UINT {
        const DWRITE_FONT_SIMULATIONS_BOLD = 1,
        const DWRITE_FONT_SIMULATIONS_OBLIQUE = 2
    }
}

impl Default for FontSimulations {
    fn default() -> FontSimulations { FontSimulations::empty() }
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FontWeight {
    Thin = 100,
    ExtraLight = 200,
    Light = 300,
    SemiLight = 350,
    Normal = 400,
    Medium = 500,
    SemiBold = 600,
    Bold = 700,
    ExtraBold = 800,
    Black = 900,
    ExtraBlack = 950,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FontStretch {
    Undefined = 0,
    UltraCondensed = 1,
    ExtraCondensed = 2,
    Condensed = 3,
    SemiCondensed = 4,
    Normal = 5,
    SemiExpanded = 6,
    Expanded = 7,
    ExtraExpanded = 8,
    UltraExpanded = 9
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FontStyle {
    Normal,
    Oblique,
    Italic
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InformationalStringID {
    None,
    CopyrightNotice,
    VersionStrings,
    Trademark,
    Manufacturer,
    Designer,
    DesignerURL,
    Description,
    FontVendorURL,
    LicenseDescription,
    LicenseInfoURL,
    Win32FamilyNames,
    Win32SubfamilyNames,
    PreferredFamilyNames,
    PreferredSubfamilyNames,
    SampleText,
    FullName,
    PostscriptName,
    PostscriptCidName
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FactoryType {
    Shared,
    Isolated
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PixelGeometry {
    Flat,
    RGB,
    BGR
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RenderingMode {
    Default,
    Aliased,
    GDIClassic,
    GDINatural,
    Natural,
    NaturalSymmetric,
    Outline
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReadingDirection {
    LeftToRight = 0,
    RightToLeft = 1,
    TopToBottom = 2,
    BottomToTop = 3
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FlowDirection {
    TopToBottom = 0,
    BottomToTop = 1,
    LeftToRight = 2,
    RightToLeft = 3
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TextAlignment {
    Leading,
    Trailing,
    Center,
    Justified
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ParagraphAlignment {
    Near,
    Far,
    Center
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WordWrapping {
    Wrap = 0,
    NoWrap = 1,
    EmergencyBreak = 2,
    WholeWord = 3,
    Character = 4
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LineSpacingMethod {
    Default,
    Uniform
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TrimmingGranularity {
    None,
    Character,
    Word
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FontFeatureTag {
    AlternativeFractions = 0x63726661, // 'afrc'
    PetiteCapitalsFromCapitals = 0x63703263, // 'c2pc'
    SmallCapitalsFromCapitals = 0x63733263, // 'c2sc'
    ContextualAlternates = 0x746c6163, // 'calt'
    CaseSensitiveForms = 0x65736163, // 'case'
    GlyphCompositionDecomposition = 0x706d6363, // 'ccmp'
    ContextualLigatures = 0x67696c63, // 'clig'
    CapitalSpacing = 0x70737063, // 'cpsp'
    ContextualSwash = 0x68777363, // 'cswh'
    CursivePositioning = 0x73727563, // 'curs'
    Default = 0x746c6664, // 'dflt'
    DiscretionaryLigatures = 0x67696c64, // 'dlig'
    ExpertForms = 0x74707865, // 'expt'
    Fractions = 0x63617266, // 'frac'
    FullWidth = 0x64697766, // 'fwid'
    HalfForms = 0x666c6168, // 'half'
    HalantForms = 0x6e6c6168, // 'haln'
    AlternateHalfWidth = 0x746c6168, // 'halt'
    HistoricalForms = 0x74736968, // 'hist'
    HorizontalKanaAlternates = 0x616e6b68, // 'hkna'
    HistoricalLigatures = 0x67696c68, // 'hlig'
    HalfWidth = 0x64697768, // 'hwid'
    HojoKanjiForms = 0x6f6a6f68, // 'hojo'
    JIS04Forms = 0x3430706a, // 'jp04'
    JIS78Forms = 0x3837706a, // 'jp78'
    JIS83Forms = 0x3338706a, // 'jp83'
    JIS90Forms = 0x3039706a, // 'jp90'
    Kerning = 0x6e72656b, // 'kern'
    StandardLigatures = 0x6167696c, // 'liga'
    LiningFigures = 0x6d756e6c, // 'lnum'
    LocalizedForms = 0x6c636f6c, // 'locl'
    MarkPositioning = 0x6b72616d, // 'mark'
    MathematicalGreek = 0x6b72676d, // 'mgrk'
    MarkToMarkPositioning = 0x6b6d6b6d, // 'mkmk'
    AlternateAnnotationForms = 0x746c616e, // 'nalt'
    NLCKanjiForms = 0x6b636c6e, // 'nlck'
    OldStyleFigures = 0x6d756e6f, // 'onum'
    Ordinals = 0x6e64726f, // 'ordn'
    ProportionalAlternateWidth = 0x746c6170, // 'palt'
    PetiteCapitals = 0x70616370, // 'pcap'
    ProportionalFigures = 0x6d756e70, // 'pnum'
    ProportionalWidths = 0x64697770, // 'pwid'
    QuarterWidths = 0x64697771, // 'qwid'
    RequiredLigatures = 0x67696c72, // 'rlig'
    RubyNotationForms = 0x79627572, // 'ruby'
    StylisticAlternates = 0x746c6173, // 'salt'
    ScientificInferiors = 0x666e6973, // 'sinf'
    SmallCapitals = 0x70636d73, // 'smcp'
    SimplifiedForms = 0x6c706d73, // 'smpl'
    StylisticSet1 = 0x31307373, // 'ss01'
    StylisticSet2 = 0x32307373, // 'ss02'
    StylisticSet3 = 0x33307373, // 'ss03'
    StylisticSet4 = 0x34307373, // 'ss04'
    StylisticSet5 = 0x35307373, // 'ss05'
    StylisticSet6 = 0x36307373, // 'ss06'
    StylisticSet7 = 0x37307373, // 'ss07'
    StylisticSet8 = 0x38307373, // 'ss08'
    StylisticSet9 = 0x39307373, // 'ss09'
    StylisticSet10 = 0x30317373, // 'ss10'
    StylisticSet11 = 0x31317373, // 'ss11'
    StylisticSet12 = 0x32317373, // 'ss12'
    StylisticSet13 = 0x33317373, // 'ss13'
    StylisticSet14 = 0x34317373, // 'ss14'
    StylisticSet15 = 0x35317373, // 'ss15'
    StylisticSet16 = 0x36317373, // 'ss16'
    StylisticSet17 = 0x37317373, // 'ss17'
    StylisticSet18 = 0x38317373, // 'ss18'
    StylisticSet19 = 0x39317373, // 'ss19'
    StylisticSet20 = 0x30327373, // 'ss20'
    Subscript = 0x73627573, // 'subs'
    Superscript = 0x73707573, // 'sups'
    Swash = 0x68737773, // 'swsh'
    Titling = 0x6c746974, // 'titl'
    TraditionalNameForms = 0x6d616e74, // 'tnam'
    TabularFigures = 0x6d756e74, // 'tnum'
    TraditionalForms = 0x64617274, // 'trad'
    ThirdWidths = 0x64697774, // 'twid'
    Unicase = 0x63696e75, // 'unic'
    VerticalWriting = 0x74726576, // 'vert'
    VerticalAlternatesAndRotation = 0x32747276, // 'vrt2'
    SlashedZero = 0x6f72657a // 'zero'
}

impl FontFeatureTag {
    /// Create a custom OpenType tag from a 4-character byte string.
    ///
    /// Marked as unsafe as it allows you to create discriminants that are not
    /// part of the enum definition which is undefined behaviour.
    ///
    /// ```
    /// use directx_sys::dwrite::FontFeatureTag;
    /// let rust_tag = unsafe { FontFeatureTag::from_chars(b"rust") };
    /// ```
    pub unsafe fn from_chars(tag: &[u8; 4]) -> FontFeatureTag {
        ::std::mem::transmute(
            ((tag[3] as u32) << 24) | ((tag[2] as u32) << 16) |
            ((tag[1] as u32) << 8) | tag[0] as u32)
    }

    pub fn to_chars(&self) -> [u8; 4] {
        let x = *self as u32;
        [x as u8, (x >> 8) as u8, (x >> 16) as u8, (x >> 24) as u8]
    }
}

#[test]
fn dwrite_test_font_feature_tag() {
    let afrc = b"afrc";
    let afrc_tag = unsafe { FontFeatureTag::from_chars(afrc) };
    assert_eq!(afrc_tag, FontFeatureTag::AlternativeFractions);
    assert_eq!(&FontFeatureTag::AlternativeFractions.to_chars(), afrc);

    let abcd = b"abcd";
    let abcd_tag = unsafe { FontFeatureTag::from_chars(abcd) };
    assert_eq!(&abcd_tag.to_chars(), abcd);
    assert!(afrc_tag != abcd_tag);
}

bitflags! {
    #[repr(C)]
    flags ScriptShapes: UINT {
        const DWRITE_SCRIPT_SHAPES_NO_VISUAL = 1
    }
}

impl Default for ScriptShapes {
    fn default() -> ScriptShapes { ScriptShapes::empty() }
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BreakCondition {
    Neutral,
    CanBreak,
    MayNotBreak,
    MustBreak
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NumberSubstitutionMethod {
    FromCulture,
    Contextual,
    None,
    National,
    Traditional
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TextureType {
    Aliased1x1,
    ClearType3x1
}
