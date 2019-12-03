//{"message":"log message","timestamp":"2019-11-13T17:46:06.244484-08:00","fields":[{"key":"0","value":"b"},{"key":"1","value":"GxL"},{"key":"2","value":"7"},{"key":"3","value":"O"},{"key":"4","value":"O1Iẞ"},{"key":"5","value":"5"},{"key":"6","value":"rv6"},{"key":"7","value":"•"},{"key":"8","value":"7js"},{"key":"9","value":"4"},{"key":"10","value":"uJ"},{"key":"11","value":"Dv"},{"key":"12","value":"c"},{"key":"13","value":"F"},{"key":"14","value":"O"},{"key":"15","value":"wM"},{"key":"16","value":"P"},{"key":"17","value":"1f5Z"},{"key":"18","value":"Z"},{"key":"19","value":"46Q"},{"key":"20","value":"iPC¬"},{"key":"21","value":"Yme1"},{"key":"22","value":"5"},{"key":"23","value":"¶j"},{"key":"24","value":"B˚l7"},{"key":"25","value":"uxPm"},{"key":"26","value":"76"},{"key":"27","value":"KWr"},{"key":"28","value":"1"},{"key":"29","value":"aBƒ"},{"key":"30","value":"741"},{"key":"31","value":"91517896"},{"key":"32","value":"144040"},{"key":"33","value":"5882"},{"key":"34","value":"71866"},{"key":"35","value":"927"},{"key":"36","value":"8381832"},{"key":"37","value":"7397"},{"key":"38","value":"600888654"},{"key":"39","value":"5"},{"key":"40","value":"10458838"},{"key":"41","value":"86"},{"key":"42","value":"6625297"},{"key":"43","value":"35"},{"key":"44","value":"495632"},{"key":"45","value":"8"},{"key":"46","value":"911183"},{"key":"47","value":"872260"},{"key":"48","value":"6"},{"key":"49","value":"10"},{"key":"50","value":"6"},{"key":"51","value":"67246"},{"key":"52","value":"803338"},{"key":"53","value":"9"},{"key":"54","value":"267"},{"key":"55","value":"973"},{"key":"56","value":"60879256"},{"key":"57","value":"370504"},{"key":"58","value":"408"},{"key":"59","value":"190"}],"level":"INFO"}

use flate2::read::GzDecoder;
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::io::{BufRead, BufWriter};
use std::io::BufReader;

#[derive(Debug, Deserialize)]
struct KeyValue {
    key: String,
    value: Value,
}

#[derive(Debug, Deserialize)]
struct Data {
    fields: Vec<KeyValue>,
}

pub fn parse() -> Result<(), Box<dyn Error>> {
    let file = File::open("./test.json.gz")?;
    let gz = GzDecoder::new(file);
    let mut reader = BufReader::new(gz);
    //    let mut buff: Vec<u8> = Vec::new();
    let mut line: String = String::new();

    let mut results: HashMap<String, u64> = HashMap::new();

    while reader.read_line(&mut line)? > 0 {
        let v: Data = serde_json::from_str(&line)?;
        for field in v.fields {
            results
                .entry(field.key + " " + &field.value.to_string())
                .and_modify(|e| *e += 1)
                .or_insert(0);
        }
        line.clear()
    }
    let output = File::create("output.txt")?;
    let mut w = BufWriter::new(output);
    for (k, v) in results {
        if v > 0 {
            w.write_all(format!("{} - {}", k, v).as_bytes())?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
