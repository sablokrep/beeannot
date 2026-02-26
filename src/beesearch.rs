#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct PathFile {
    pub pathfile: String,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct DNASeq {
    pub name: String,
    pub geneid: String,
    pub annotationtype: String,
    pub start: String,
    pub end: String,
    pub idtype: String,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct DNASeqUnfold {
    pub name: String,
    pub geneid: String,
    pub annotationtype: String,
    pub start: String,
    pub end: String,
    pub idtype: String,
    pub parentid: String,
}
