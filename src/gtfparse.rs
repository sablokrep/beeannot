use crate::beesearch::DNASeq;
use crate::beesearch::DNASeqUnfold;
use crate::beesearch::PathFile;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

/*
Gaurav Sablok
codeprog@icloud.com
*/

impl PathFile {
    pub fn prepareaxum<P: AsRef<std::path::Path>>(
        &self,
        pathfile: P,
    ) -> Result<Vec<DNASeqUnfold>, Box<dyn Error>> {
        let filepath = File::open(pathfile).expect("file not present");
        let fileread = BufReader::new(filepath);
        let mut cleanvec: Vec<DNASeq> = Vec::new();
        for i in fileread.lines() {
            let line = i.expect("file not present");
            let linewidth = line.split("\t").map(|x| x.to_string()).collect::<Vec<_>>();
            cleanvec.push(DNASeq {
                name: linewidth[0].clone(),
                geneid: linewidth[1].clone(),
                annotationtype: linewidth[2].clone(),
                start: linewidth[3].clone(),
                end: linewidth[4].clone(),
                idtype: linewidth[7].clone(),
            })
        }

        let vectorunwrap = processid(cleanvec).unwrap();

        Ok(vectorunwrap)
    }
}
fn processid(inputstruct: Vec<DNASeq>) -> Result<Vec<DNASeqUnfold>, Box<dyn Error>> {
    let mutvec = inputstruct.clone();
    let mut removedstruct: Vec<DNASeqUnfold> = Vec::new();
    for i in mutvec.iter() {
        let valuemodified = i
            .idtype
            .split(";")
            .map(|x| x.to_string())
            .collect::<Vec<_>>();
        removedstruct.push(DNASeqUnfold {
            name: i.name.clone(),
            geneid: i.geneid.clone(),
            annotationtype: i.annotationtype.clone(),
            start: i.start.clone(),
            end: i.end.clone(),
            idtype: valuemodified[0].split("=").collect::<Vec<_>>()[1].to_string(),
            parentid: valuemodified[1].split("=").collect::<Vec<_>>()[1].to_string(),
        })
    }
    Ok(removedstruct)
}
