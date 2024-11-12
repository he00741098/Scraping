use base64::prelude::*;
use bytes::Bytes;
use serde_json::{json, Value};
use std::path::Path;
use tokio::fs::read_dir;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    // download_all_pdfs().await;
    // combine_all_json_files(188).await;
    // read_filter_combined_json_files().await;
    // let path = "./json/combined.json";
    // // let mut entries_read = 0;
    // let file_contents = tokio::fs::read_to_string(path).await;
    // if file_contents.is_err() {
    //     println!("Could not read file");
    //     return;
    // } else {
    //     println!("Parsing");
    // }
    // let file_contents = file_contents.unwrap();
    // let file_contents = serde_json::from_str::<Value>(&file_contents);
    // if file_contents.is_err() {
    //     println!("Could not parse");
    //     return;
    // }
    // let file_contents = file_contents.unwrap();
    // let file_contents = if let Value::Array(contents) = file_contents {
    //     contents
    // } else {
    //     println!("Not Array");
    //     return;
    // };
    // let test_array = file_contents[0].as_object().expect("To be array").keys();
    // .get("publication_info")
    // .expect("To have pub info")
    // .as_array()
    // .expect("Pub info to be array");
    // test_array.for_each(|f| println!("{}", f));
    // read_character_slice(48987394 - 500, 48987394 + 500).await;
    // format_to_xmls().await;
    upload_all().await;
    // partition_files().await;
    // get_list_to_delete().await;
}

async fn read_character_slice(start: usize, end: usize) {
    let file = tokio::fs::read_to_string("./json/combined.json")
        .await
        .unwrap();
    let file = file.to_string();
    println!(
        "OUT OF {} CHARACTERS:\n...|{}|...",
        file.len(),
        &file[start..end]
    );
}

async fn get_list_to_delete() {
    let file = tokio::fs::read_to_string("submissions.json").await.unwrap();
    let entries = serde_json::from_str::<Value>(&file).unwrap();
    let entries = entries.as_object().unwrap();
    let items = entries.get("items").unwrap().as_array().unwrap();
    let mut deletion = Vec::with_capacity(3000);
    for i in items {
        if i.as_object()
            .unwrap()
            .get("dateSubmitted")
            .unwrap()
            .as_str()
            .unwrap()
            .contains("2024-07-26")
        {
            deletion.push(i.as_object().unwrap().get("id").unwrap().as_i64().unwrap());
        }
    }
    deletion.iter().for_each(|x| print!(" {}", x));
    println!("\n");
}

async fn partition_files() {
    let mut dir = read_dir("./xml/").await.expect("Path exists");
    let mut amount = 0;
    while let Ok(Some(entry)) = dir.next_entry().await {
        println!("Renaming: {:?}", entry.path().to_str());
        let result = std::fs::create_dir(format!("./xml/xml{}/", amount / 3000));
        let result = std::fs::rename(
            entry.path(),
            format!(
                "./xml/xml{}/{}",
                amount / 3000,
                entry.file_name().to_str().unwrap()
            ),
        );
        if result.is_err() {
            println!("Partition failed at {:?}", entry);
            return;
        }
        amount += 1;
    }
    println!("Partitioned {} entries", amount);
}

async fn upload_all() {
    let token="Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.IjI3Nzc2NjFmYTRhNjk2OWNkODI3MDA1ZjcwOWMyMjk2MzQwM2E4NzIi.cbRSeFZNL-5hPS4Irne6xUkRsVQsRfDBY4sI9ZwI5VY";
    let test_body = r##"{
    "locale": "en",
    "sectionId": 0,
    "userGroupId": 0}"##;

    let client = reqwest::ClientBuilder::new()
        // .user_agent("Mozilla/5.0 (X11; Linux x86_64; rv:128.0) Gecko/20100101 Firefox/128.0")
        .build()
        .expect("Client to be built");
    let mut total = 0;
    loop {
        println!("Submitting");
        let submissions = client
        .get("https://ayurxiv.org/index.php/ayurxiv/api/v1/submissions?status=1&count=100&apiToken=eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.IjI3Nzc2NjFmYTRhNjk2OWNkODI3MDA1ZjcwOWMyMjk2MzQwM2E4NzIi.cbRSeFZNL-5hPS4Irne6xUkRsVQsRfDBY4sI9ZwI5VY")
        .send()
        .await.unwrap().text().await.unwrap();
        // println!("{:?}", submissions);
        let entries = serde_json::from_str::<Value>(&submissions).unwrap();
        let entries = entries.as_object().unwrap();
        let items = entries.get("items").unwrap().as_array().unwrap();
        total = entries.get("itemsMax").unwrap().as_i64().unwrap();
        if total == 0 {
            println!("Zero");
            return;
        }
        for i in items {
            let obj = i.as_object().unwrap().get("id").unwrap().as_i64().unwrap();
            let pub_id = i
                .as_object()
                .unwrap()
                .get("currentPublicationId")
                .unwrap()
                .as_i64()
                .unwrap();
            let response = client.put(format!("https://ayurxiv.org/index.php/ayurxiv/api/v1/submissions/{}/publications/{}/publish?apiToken=eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.IjI3Nzc2NjFmYTRhNjk2OWNkODI3MDA1ZjcwOWMyMjk2MzQwM2E4NzIi.cbRSeFZNL-5hPS4Irne6xUkRsVQsRfDBY4sI9ZwI5VY", obj, pub_id)).send().await.expect("To have response");
            let response = response.text().await.unwrap();
            println!("{}\n\n", response);
        }
    }
}

// async fn upload_all_xmls() {
//     let mut dir = read_dir("./xml/").await.expect("Path exists");
//     let client = reqwest::ClientBuilder::new()
//         .user_agent("Mozilla/5.0 (X11; Linux x86_64; rv:128.0) Gecko/20100101 Firefox/128.0")
//         .build()
//         .expect("Client to be built");
//     let mut sent = 0;
//     while let Ok(Some(entry)) = dir.next_entry().await {
//         let divider = "---------------------------7023907691680357374782039712";
//         let file_contents = tokio::fs::read_to_string(entry.path()).await.unwrap();
//         let name = entry.file_name();
//         let name = name.to_str().unwrap();
//         let body = format!(
//             "{}\nContent-Disposition: form-data; name=\"name\"\n\n{}\n{}\nContent-Disposition: form-data; name=\"uploadedFile\"; filename=\"{}\"\nContent-Type: text/xml\n\n{}\n\n{}--",
//             divider, name, divider, name, file_contents, divider
//         );
//         let bodyclone = body.clone();
//         let request = client.post("https://ayurxiv.org/index.php/server/management/importexport/plugin/NativeImportExportPlugin/uploadImportXML").header("Accept", "*/*").header("Content-Type", "multipart/form-data; boundary=---------------------------7023907691680357374782039712").header("Cookie", "OPSSID=prld97t14qlt1fpa2ngh2v64fq").header("Host", "ayurxiv.org").header("Origin", "https://ayurxiv.org").header("Referer", "https://ayurxiv.org/index.php/server/management/importexport/plugin/NativeImportExportPlugin");
//         // println!("Could not read file: {:?}", entry.path());
//         let post_result = request.body(body).send().await;
//         if post_result.is_err() {
//             println!("{:?}", post_result);
//             let place = format!("Failed at {}", sent);
//             tokio::fs::write("place.txt", sent.to_string())
//                 .await
//                 .expect(&place);
//             return;
//         }
//         let response = post_result.unwrap().json::<Value>().await;
//         if response.is_err() {
//             println!("Failed to parse post result at {}\n{:?}", sent, response);
//             return;
//         }
//         let responser = response.unwrap();
//         if !responser.is_object() {
//             println!("response is not object at {} \n{}", sent, responser);
//             return;
//         }
//         let response = responser.as_object().unwrap();
//         let field = response.get("temporaryFileId");
//         if field.is_none() {
//             println!(
//                 "Could not get file id at {}, {:?}\n{}",
//                 sent, response, bodyclone
//             );
//             return;
//         }
//         let field = field.unwrap();
//         let field = match field {
//             Value::Null => {
//                 println!("Null");
//                 None
//             }
//             Value::Bool(_) => {
//                 println!("Bool");
//                 None
//             }
//             Value::Number(x) => Some(x.to_string()),
//             Value::String(x) => Some(x.to_string()),
//             Value::Array(_) => {
//                 println!("Array");
//                 None
//             }
//             Value::Object(_) => {
//                 println!("Object");
//                 None
//             }
//         };
//         if field.is_none() {
//             println!("Is none!!!");
//             return;
//         }
//         let field = field.unwrap();
//         let request = client.post("https://ayurxiv.org/index.php/server/management/importexport/plugin/NativeImportExportPlugin/importBounce").header("Accept", "*/*").header("Content-Type", "application/x-www-form-urlencoded; charset=UTF-8").header("Cookie", "OPSSID=prld97t14qlt1fpa2ngh2v64fq").header("X-Requested-With", "XMLHttpRequest").header("Priority", "u=0");
//         let request = request.body(
//             "csrfToken=a04af340f797adb55a2b8c03c8b84935&temporaryFileId=70&submitFormButton=",
//         );
//         let response = request.send().await;
//         if response.is_err() {
//             println!("Is err");
//         }
//         sent += 1;
//     }
// }

async fn format_to_xmls() {
    let starterxml = tokio::fs::read_to_string("./starterxml.txt").await.unwrap();
    let keyword_format = "<keyword>##KEYWORD##</keyword>";
    let biography_template = "<biography>##BIOGRAPHY##</biography>";
    let author_format = r####"
      <author id="##AUTHOR_NUM##" include_in_browse="true" seq="0" user_group_ref="Author">
        <givenname locale="en">##FIRST_NAME##</givenname>
        <familyname locale="en">##LAST_NAME##</familyname>
        <affiliation locale="en">##AFFILIATION##</affiliation>
        <country>##COUNTRY##</country>
        <email>##EMAIL##</email>
        ##BIOGRAPHY##
      </author>
"####;
    let path = "./json/combined.json";
    let file_contents = tokio::fs::read_to_string(path).await;
    if file_contents.is_err() {
        println!("Could not read file");
        return;
    } else {
        println!("Parsing");
    }
    let file_contents = file_contents.unwrap();
    let file_contents = serde_json::from_str::<Value>(&file_contents);
    if file_contents.is_err() {
        println!("Could not parse: {:?}", file_contents);
        return;
    }
    let file_contents = file_contents.unwrap();
    let file_contents = if let Value::Array(contents) = file_contents {
        contents
    } else {
        println!("Not Array");
        return;
    };
    let mut keyword_skipped = 0;
    for test_run in file_contents {
        let test_run = test_run.as_object().unwrap();
        let pmcid = test_run
            .get("PMCID")
            .unwrap()
            .as_str()
            .unwrap()
            .replace("PMCID: ", "");
        let pdf_name = format!("{}.pdf", pmcid);
        let pdf = tokio::fs::read(format!("./pdf/{}", pdf_name))
            .await
            .unwrap();
        let pdf_size = pdf.len();
        let pdf_embed = BASE64_STANDARD.encode(pdf);
        let doi = test_run.get("doi").unwrap().as_str().unwrap();
        let title = replace_invalid_characters(test_run.get("title").unwrap().as_str().unwrap());
        let abs = test_run.get("abstract_text");
        if abs.is_none() {
            continue;
        }

        let abs = abs.unwrap();
        let abs = if abs.is_array() {
            abs.as_array()
                .unwrap()
                .iter()
                .map(|f| replace_invalid_characters(f.as_str().unwrap()))
                .filter(|x| !x.contains("Abstract") || x.len() > 10)
                .filter(|x| !x.contains("Background") || x.len() > 11)
                .filter(|x| !x.contains("Keywords: ") || x.len() > 11)
                .collect::<Vec<String>>()
                .join("\n")
        } else if abs.is_string() {
            replace_invalid_characters(abs.as_str().unwrap())
        } else {
            println!("Skipping. no Abstract");
            continue;
        };
        let keywords = test_run.get("keywords").unwrap();
        let keywords = if keywords.is_array() {
            let keywords = keywords
                .as_array()
                .unwrap()
                .iter()
                .map(|key| {
                    let copy = keyword_format.replace(
                        "##KEYWORD##",
                        &replace_invalid_characters(key.as_str().unwrap()),
                    );
                    copy
                })
                .collect::<Vec<String>>();
            if keywords.len() < 3 || keywords.len() > 12 {
                // println!("Keywords: {}", keywords.len());
                keyword_skipped += 1;
                continue;
            }
            keywords.join("\n")
        } else {
            // println!("Keywords is not Array: {:?}", keywords);
            continue;
        };
        //{"name":"Christian S. Kessler","attributes":[""],"email":"c.kessler@immanuel.de"}
        let authors = test_run
            .get("publication_info")
            .unwrap()
            .as_array()
            .unwrap()
            .iter()
            .map(|author| {
                // println!("{}", author);
                author.as_object().unwrap()
            })
            .map(|author| Author {
                name: author.get("name").unwrap().as_str().unwrap(),
                attributes: author
                    .get("attributes")
                    .unwrap()
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|attr| replace_invalid_characters(attr.as_str().unwrap()))
                    .map(|attr| {
                        let mut chars = attr.chars();
                        let mut word = attr.clone();
                        while let Some(x) = chars.next() {
                            if x.is_numeric() || !x.is_alphabetic() {
                                continue;
                            } else {
                                word = format!("{}{}", x, chars.collect::<String>());
                                break;
                            }
                        }
                        word
                    })
                    .collect(),
                email: replace_invalid_characters(author.get("email").unwrap().as_str().unwrap()),
                country: replace_invalid_characters(
                    author
                        .get("country")
                        .unwrap_or(&Value::String("US".to_string()))
                        .as_str()
                        .unwrap(),
                ),
            })
            .collect::<Vec<Author>>();
        let mut author_list = Vec::with_capacity(authors.len());
        for author_num in 0..authors.len() {
            let author = &authors[author_num];
            let copy = author_format.replace(
                "##FIRST_NAME##",
                &replace_invalid_characters(author.name.split(' ').collect::<Vec<&str>>()[0]),
            );
            let copy = copy.replace(
                "##LAST_NAME##",
                &replace_invalid_characters(
                    author
                        .name
                        .split(' ')
                        .collect::<Vec<&str>>()
                        .last()
                        .unwrap(),
                ),
            );
            let author_index = author_num + 1;
            let copy = copy.replace("##AUTHOR_NUM##", &format!("{}", author_index));

            let copy = copy.replace("##COUNTRY##", &replace_invalid_characters(&author.country));
            let copy = copy.replace("##EMAIL##", &author.email);
            let copy = if !author.attributes.is_empty() {
                let copy = copy.replace(
                    "##AFFILIATION##",
                    &replace_invalid_characters(&author.attributes[0]),
                );
                if author.attributes.len() > 1 {
                    let biographies = author.attributes[1..]
                        .iter()
                        // .skip(1)
                        .map(|attr| {
                            biography_template
                                .replace("##BIOGRAPHY##", &replace_invalid_characters(attr))
                        })
                        .collect::<Vec<String>>()
                        .join("\n");
                    copy.replace("##BIOGRAPHY##", &biographies)
                } else {
                    copy.replace("##BIOGRAPHY##", "")
                }
            } else {
                let copy = copy.replace("##AFFILIATION##", "");
                copy.replace("##BIOGRAPHY##", "")
            };
            // let copy = copy.replace("##BIOGRAPHY##", to);
            author_list.push(copy);
        }
        let authors = author_list.join("\n");
        let base_temp = &*starterxml;
        let base_temp = base_temp.replace("##PDF_NAME##", &pdf_name);
        let base_temp = base_temp.replace("##FILE_SIZE##", &format!("{}", pdf_size));
        let base_temp = base_temp.replace("##DOI##", doi);
        let base_temp = base_temp.replace("##TITLE##", &*title);
        let base_temp = base_temp.replace("##ABSTRACT##", &abs);
        let base_temp = base_temp.replace("##KEYWORDS##", &keywords);
        let base_temp = base_temp.replace("##AUTHORS##", &authors);
        // let base_temp = base_temp.replace('&', "and");
        let base_temp = base_temp.replace("##EMBED##", &pdf_embed);
        let file_name = format!("./xml/{}", (&pdf_name).replace(".pdf", ".xml"));
        tokio::fs::write(file_name, base_temp.as_bytes())
            .await
            .unwrap();
    }

    println!("Skipped {} due to insufficient keywords", keyword_skipped);
}
struct Author<'a> {
    name: &'a str,
    attributes: Vec<String>,
    email: String,
    country: String,
}

async fn read_filter_combined_json_files() {
    let countries = [
        ["AF", "Afghanistan"],
        ["AL", "Albania"],
        ["DZ", "Algeria"],
        ["AS", "American Samoa"],
        ["AD", "Andorra"],
        ["AO", "Angola"],
        ["AI", "Anguilla"],
        ["AQ", "Antarctica"],
        ["AG", "Antigua###Barbuda"],
        ["AR", "Argentina"],
        ["AM", "Armenia"],
        ["AW", "Aruba"],
        ["AU", "Australia"],
        ["AT", "Austria"],
        ["AZ", "Azerbaijan"],
        ["BS", "Bahamas"],
        ["BH", "Bahrain"],
        ["BD", "Bangladesh"],
        ["BB", "Barbados"],
        ["BY", "Belarus"],
        ["BE", "Belgium"],
        ["BZ", "Belize"],
        ["BJ", "Benin"],
        ["BM", "Bermuda"],
        ["BT", "Bhutan"],
        ["BO", "Bolivia"],
        ["BQ", "Bonaire###Sint Eustatius"],
        ["BA", "Bosnia###Herzegovina"],
        ["BW", "Botswana"],
        ["BV", "Bouvet"],
        ["BR", "Brazil"],
        ["IO", "British Indian Ocean Territory"],
        ["BN", "Brunei###Darussalam"],
        ["BG", "Bulgaria"],
        ["BF", "Burkina###Faso"],
        ["BI", "Burundi"],
        ["CV", "Cabo Verde"],
        ["KH", "Cambodia"],
        ["CM", "Cameroon"],
        ["CA", "Canada"],
        ["KY", "Cayman Islands"],
        ["CF", "Central African Republic"],
        ["TD", "Chad"],
        ["CL", "Chile"],
        ["CN", "China"],
        ["CX", "Christmas Island"],
        ["CC", "Cocos Islands"],
        ["CO", "Colombia"],
        ["KM", "Comoros"],
        ["CG", "Congo"],
        ["CD", "Democratic Republic of the Congo"],
        ["CK", "Cook Islands"],
        ["CR", "Costa Rica"],
        ["HR", "Croatia"],
        ["CU", "Cuba"],
        ["CW", "Curaçao"],
        ["CY", "Cyprus"],
        ["CZ", "Czechia"],
        ["CI", "Côte d'Ivoire"],
        ["DK", "Denmark"],
        ["DJ", "Djibouti"],
        ["DM", "Dominica"],
        ["DO", "Dominican Republic"],
        ["EC", "Ecuador"],
        ["EG", "Egypt"],
        ["SV", "El Salvador"],
        ["GQ", "Equatorial Guinea"],
        ["ER", "Eritrea"],
        ["EE", "Estonia"],
        ["SZ", "Eswatini"],
        ["ET", "Ethiopia"],
        ["FK", "Falkland Islands"],
        ["FO", "Faroe Islands"],
        ["FJ", "Fiji"],
        ["FI", "Finland"],
        ["FR", "France"],
        ["GF", "French Guiana"],
        ["PF", "French Polynesia"],
        ["TF", "French Southern Territories"],
        ["GA", "Gabon"],
        ["GM", "Gambia"],
        ["GE", "Georgia"],
        ["DE", "Germany"],
        ["GH", "Ghana"],
        ["GI", "Gibraltar"],
        ["GR", "Greece"],
        ["GL", "Greenland"],
        ["GD", "Grenada"],
        ["GP", "Guadeloupe"],
        ["GU", "Guam"],
        ["GT", "Guatemala"],
        ["GG", "Guernsey"],
        ["GN", "Guinea"],
        ["GW", "Bissau"],
        ["GY", "Guyana"],
        ["HT", "Haiti"],
        ["HM", "Heard Island and McDonald Islands"],
        ["VA", "Holy See###Vatican City State"],
        ["HN", "Honduras"],
        ["HK", "Hong Kong"],
        ["HU", "Hungary"],
        ["IS", "Iceland"],
        ["IN", "India"],
        ["ID", "Indonesia"],
        ["IR", "Iran"],
        ["IQ", "Iraq"],
        ["IE", "Ireland"],
        ["IM", "Isle of Man"],
        ["IL", "Israel"],
        ["IT", "Italy"],
        ["JM", "Jamaica"],
        ["JP", "Japan"],
        ["JE", "Jersey"],
        ["JO", "Jordan"],
        ["KZ", "Kazakhstan"],
        ["KE", "Kenya"],
        ["KI", "Kiribati"],
        ["KP", "North Korea"],
        ["KR", "South Korea"],
        ["KW", "Kuwait"],
        ["KG", "Kyrgyzstan"],
        ["LA", "Lao People's Democratic Republic"],
        ["LV", "Latvia"],
        ["LB", "Lebanon"],
        ["LS", "Lesotho"],
        ["LR", "Liberia"],
        ["LY", "Libya"],
        ["LI", "Liechtenstein"],
        ["LT", "Lithuania"],
        ["LU", "Luxembourg"],
        ["MO", "Macao"],
        ["MG", "Madagascar"],
        ["MW", "Malawi"],
        ["MY", "Malaysia"],
        ["MV", "Maldives"],
        ["ML", "Mali"],
        ["MT", "Malta"],
        ["MH", "Marshall Islands"],
        ["MQ", "Martinique"],
        ["MR", "Mauritania"],
        ["MU", "Mauritius"],
        ["YT", "Mayotte"],
        ["MX", "Mexico"],
        ["FM", "Micronesia"],
        ["MD", "Moldova"],
        ["MC", "Monaco"],
        ["MN", "Mongolia"],
        ["ME", "Montenegro"],
        ["MS", "Montserrat"],
        ["MA", "Morocco"],
        ["MZ", "Mozambique"],
        ["MM", "Myanmar"],
        ["NA", "Namibia"],
        ["NR", "Nauru"],
        ["NP", "Nepal"],
        ["NL", "Netherlands"],
        ["NC", "New Caledonia"],
        ["NZ", "New Zealand"],
        ["NI", "Nicaragua"],
        ["NE", "Niger"],
        ["NG", "Nigeria"],
        ["NU", "Niue"],
        ["NF", "Norfolk Island"],
        ["MK", "North Macedonia"],
        ["MP", "Northern Mariana"],
        ["NO", "Norway"],
        ["OM", "Oman"],
        ["PK", "Pakistan"],
        ["PW", "Palau"],
        ["PS", "Palestine"],
        ["PA", "Panama"],
        ["PG", "Papua"],
        ["PY", "Paraguay"],
        ["PE", "Peru"],
        ["PH", "Philippines"],
        ["PN", "Pitcairn"],
        ["PL", "Poland"],
        ["PT", "Portugal"],
        ["PR", "Puerto Rico"],
        ["QA", "Qatar"],
        ["RO", "Romania"],
        ["RU", "Russia"],
        ["RW", "Rwanda"],
        ["RE", "Réunion"],
        ["BL", "Saint Barthélemy"],
        ["SH", "Saint Helena"],
        ["KN", "Saint Kitts###Nevis"],
        ["LC", "Saint Lucia"],
        ["MF", "Saint Martin"],
        ["PM", "Saint Pierre###Miquelon"],
        ["VC", "Saint Vincent###Grenadines"],
        ["WS", "Samoa"],
        ["SM", "San Marino"],
        ["ST", "Sao Tome and Principe"],
        ["SA", "Saudi Arabia"],
        ["SN", "Senegal"],
        ["RS", "Serbia"],
        ["SC", "Seychelles"],
        ["SL", "Sierra Leone"],
        ["SG", "Singapore"],
        ["SX", "Sint Maarten"],
        ["SK", "Slovakia"],
        ["SI", "Slovenia"],
        ["SB", "Solomon Islands"],
        ["SO", "Somalia"],
        ["ZA", "South Africa"],
        ["GS", "South Georgia and the South Sandwich Islands"],
        ["SS", "South Sudan"],
        ["ES", "Spain"],
        ["LK", "Sri Lanka"],
        ["SD", "Sudan"],
        ["SR", "Suriname"],
        ["SJ", "Svalbard###Jan Mayen"],
        ["SE", "Sweden"],
        ["CH", "Switzerland"],
        ["SY", "Syrian Arab Republic"],
        ["TW", "Taiwan"],
        ["TJ", "Tajikistan"],
        ["TZ", "Tanzania"],
        ["TH", "Thailand"],
        ["TL", "Timor-Leste"],
        ["TG", "Togo"],
        ["TK", "Tokelau"],
        ["TO", "Tonga"],
        ["TT", "Trinidad###Tobago"],
        ["TN", "Tunisia"],
        ["TM", "Turkmenistan"],
        ["TC", "Turks and Caicos Islands"],
        ["TV", "Tuvalu"],
        ["TR", "Türkiye"],
        ["UG", "Uganda"],
        ["UA", "Ukraine"],
        ["AE", "United Arab Emirates"],
        ["GB", "United Kingdom"],
        ["US", "United States"],
        ["UM", "United States Minor Outlying Islands"],
        ["UY", "Uruguay"],
        ["UZ", "Uzbekistan"],
        ["VU", "Vanuatu"],
        ["VE", "Venezuela"],
        ["VN", "Viet Nam"],
        ["VG", "Virgin Islands, British"],
        ["VI", "Virgin Islands, U.S."],
        ["WF", "Wallis###Futuna"],
        ["EH", "Western Sahara"],
        ["YE", "Yemen"],
        ["ZM", "Zambia"],
        ["ZW", "Zimbabwe"],
        ["AX", "Åland Islands"],
    ];

    let client = reqwest::ClientBuilder::new().user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/80.0.3987.163 Safari/537.36").build().expect("Client to be built");
    let path = "./json/combined.json";
    let mut entries_read = 0;
    let file_contents = tokio::fs::read_to_string(path).await;
    if file_contents.is_err() {
        println!("Could not read file: {:?}", file_contents);
        return;
    } else {
        println!("Parsing");
    }
    let file_contents = file_contents.unwrap();
    let file_contents = serde_json::from_str::<Value>(&file_contents);
    if file_contents.is_err() {
        println!("Could not parse: {:?}", file_contents);
        return;
    }
    let file_contents = file_contents.unwrap();
    let file_contents = if let Value::Array(contents) = file_contents {
        contents
    } else {
        println!("Not Array");
        return;
    };
    file_contents[0]
        .as_object()
        .unwrap()
        .keys()
        .for_each(|f| println!("-{}", f));
    let mut removed = 0;
    let mut entries_downloaded = 0;
    let mut entries_missed = 0;
    let mut accepted_entries: Vec<Value> = Vec::with_capacity(18000);
    for entry in file_contents {
        entries_read += 1;
        //requirements too pass: downloaded pdf and authors
        if let Value::Object(mut obj) = entry {
            let pdf_link = obj.get("pdf_link").expect("To have pdf link").clone();
            let abs = obj.get("abstract_text");
            if let Some(Value::String(x)) = abs {
                if !x.to_lowercase().contains("ayurv")
                    && !x.to_lowercase().contains("traditional")
                    && !x.to_lowercase().contains("india")
                    && !x.to_lowercase().contains("system of medicine")
                {
                    removed += 1;
                    println!("Removed For No Abstract String");
                    continue;
                }
            } else if let Some(Value::Array(x)) = abs {
                let stringy = x
                    .iter()
                    .map(|entry| entry.as_str())
                    .filter(|x| x.is_some())
                    .map(|x| x.unwrap())
                    .map(|x| x.to_owned())
                    .collect::<Vec<String>>()
                    .join("\n");
                if !stringy.to_lowercase().contains("ayurv")
                    && !stringy.to_lowercase().contains("traditional")
                    && !stringy.to_lowercase().contains("india")
                    && !stringy.to_lowercase().contains("system of medicine")
                {
                    removed += 1;
                    println!("Removed For No Abstract Array");
                    continue;
                }
            } else {
                println!("Removed For No Abstract");
                removed += 1;
                continue;
            }

            let pmcid = obj
                .get("PMCID")
                .expect("PMCID to exist")
                .as_str()
                .unwrap()
                .replace("PMCID: ", "");

            let authors = obj
                .get_mut("publication_info")
                .expect("To have author array");
            let authors = if !authors.is_array() {
                // [{"attributes":["\n1Laboratory No. 3, National Centre for Cell Science, Pune University Campus, Ganeshkhind Road, Pune, 411007 India "],"country":"IN","email":"random@sweep.rs","name":"Himanshu Kumar"}
                // println!("Authors is not array!: {:?}", authors);
                //attempt to create array
                let authors = authors
                    .as_str()
                    .unwrap_or("None")
                    .split(",")
                    .filter(|x| x != &"None")
                    .map(|x| x.trim())
                    .map(|x| {
                        let value = json!({
                            "attributes": [],
                            "country": "US",
                            "email": "random@sweep.rs",
                            "name": x
                        });
                        // println!("Mapped Value: {:?}", value);
                        value
                    })
                    .collect::<Vec<Value>>();
                // println!("Array :{:?}", authors);
                if !authors.is_empty() {
                    // println!("fixed");
                    authors
                } else {
                    println!("Not fixed");
                    continue;
                }
            } else {
                std::mem::take(authors.as_array_mut().unwrap())
            };
            if !pdf_link.is_string() {
                // println!("Pdf link is not string!");
                continue;
            }
            let pdf_link = pdf_link.as_str().unwrap();
            if authors.is_empty() {
                // println!("No Authors!");
                removed += 1;
                continue;
            } else if !pdf_link.contains("https") {
                // println!("No Link!");
                removed += 1;
                continue;
            } else if authors.len() > 10 {
                removed += 1;
                continue;
            }
            //check if file exists;
            if !Path::new(&format!("./pdf/{}.pdf", pmcid)).exists() {
                println!(
                    "File not found! {}.pdf\nAttempting to download it now...\nLink: {}",
                    pmcid, pdf_link
                );
                entries_missed += 1;
                let response = client.get(pdf_link).send().await;
                if response.is_err() {
                    println!("--Get response is error!: {:?}", response);
                    continue;
                }
                let response = response.unwrap().bytes().await;
                if response.is_err() {
                    println!("--Get response Bytes is error!");
                    continue;
                }
                // println!("Response: {:?}", response);
                let response = response.unwrap();
                println!("--Response size: {}", response.len());
                let file_path = format!("./pdf/{}.pdf", pmcid);
                let write_result = tokio::fs::write(&file_path, response).await;
                if write_result.is_err() {
                    println!("--Write Failed!!, {}", file_path);
                } else {
                    entries_downloaded += 1;
                }
            }

            //entry is qualified to be accepted - Now add country(defualt to U.S) and change null
            //emails to random@sweep.rs
            let mut revised_authors = vec![];
            for mut author in authors {
                let author = author.as_object_mut().expect("Author to be object");
                //authors is an array of objects
                let email = author
                    .get("email")
                    .expect("Author object to have email string")
                    .clone();
                let attributes = author
                    .get_mut("attributes")
                    .expect("Author object to have attributes array")
                    .as_array()
                    .expect("Author obj to contain attribute array")
                    .clone();
                if !email.is_string() {
                    author["email"] = Value::String("random@sweep.rs".to_string());
                } else if let Value::String(e) = email {
                    if e == "null" || e == "none" {
                        author["email"] = Value::String("random@sweep.rs".to_string());
                    }
                }
                for attribute in attributes {
                    if let Value::String(attr) = attribute {
                        for [code, country] in countries {
                            if attr.contains(code) || attr.contains(country) {
                                author
                                    .insert("country".to_string(), Value::String(code.to_string()));
                            }
                        }
                        if author.get("country").is_none() {
                            author.insert("country".to_string(), Value::String("US".to_string()));
                        }
                    } else {
                        println!("Attribute Isn't String!!: {:?}", attribute);
                    }
                }
                revised_authors.push(Value::Object(std::mem::take(author)));
            }
            obj.insert(
                "publication_info".to_string(),
                Value::Array(revised_authors),
            );
            // println!("Entry: {:?}", obj);
            accepted_entries.push(Value::Object(obj));
        } else {
            println!("Entry is not Object!!!!");
        }
    }
    println!("Entries Read: {}\nEntries Pushed: {}\nEntries Removed: {}\nEntries Downloaded: {}\nEntries Missing: {}", entries_read, accepted_entries.len(), removed, entries_downloaded, entries_missed);
    let stringed = serde_json::to_string(&Value::Array(accepted_entries));
    if stringed.is_ok() {
        let write_result = tokio::fs::write("./json/combined.json", stringed.unwrap()).await;
        if write_result.is_err() {
            println!("Write failed...");
        }
    }
}

async fn combine_all_json_files(files: i32) {
    let path = "./json/";
    let mut dir = read_dir(path).await.expect("Path exists");
    let mut entries_read = 0;
    let mut article_array: Vec<String> = Vec::with_capacity(17699);
    article_array.push("[".to_string());
    while let Ok(Some(entry)) = dir.next_entry().await {
        let file_contents = tokio::fs::read_to_string(entry.path()).await;
        if file_contents.is_err() {
            println!("Could not read file: {:?}", entry.path());
            continue;
        } else {
            println!("Parsing: {:?}", entry.path());
        }
        let file_contents = file_contents.unwrap();
        //remove first and last characters (the brackets)
        let chars = &*file_contents;
        let mut chars = chars.chars();
        chars.next();
        chars.next_back();
        let value = if entries_read >= files - 1 {
            println!("Last File reached!!");
            chars.as_str().to_string()
        } else {
            format!("{},", chars.as_str())
        };
        if value.len() > 1 {
            article_array.push(value);
        } else {
            println!("No Length Value!, Skipping: {}", value);
        }
        entries_read += 1;
    }
    println!("Entries Read: {}", entries_read);
    article_array.push("]".to_string());
    let combined = article_array.join("");
    let file_path = format!("{}combined.json", path);
    let write_result = tokio::fs::write(file_path, combined.as_bytes()).await;
    if write_result.is_err() {
        println!("Write Failed!!");
    }
}

struct WriteRequest {
    file_name: String,
    bytes: Bytes,
}

async fn write_to_file(mut rx: mpsc::Receiver<WriteRequest>, path: &str) {
    while let Some(thing) = rx.recv().await {
        let file_path = format!("{}{}.pdf", path, thing.file_name);
        let write_result = tokio::fs::write(file_path, thing.bytes).await;
        if write_result.is_err() {
            println!("Write Failed!!, {}", thing.file_name);
        }
    }
}

async fn download_all_pdfs() {
    let path = Path::new("./json/");
    let mut dir = read_dir(path).await.expect("Path exists");
    let mut entries_read = 0;
    let (tx, rx) = mpsc::channel::<WriteRequest>(100);
    tokio::spawn(async {
        write_to_file(rx, "./pdf/").await;
    });
    let client = reqwest::ClientBuilder::new().user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/80.0.3987.163 Safari/537.36").build().expect("Client to be built");
    // for _ in 1..=137 {
    //     if let Ok(Some(entry)) = dir.next_entry().await {
    //         println!("Skipping {:?}", entry);
    //     }
    // }
    while let Ok(Some(entry)) = dir.next_entry().await {
        //begin download...
        let file_contents = tokio::fs::read_to_string(entry.path()).await;
        if file_contents.is_err() {
            println!("Could not read file: {:?}", entry.path());
            continue;
        } else {
            println!("Parsing: {:?}", entry.path());
        }
        let file_contents = file_contents.unwrap();
        let parse = serde_json::from_str::<Value>(&file_contents);
        if let Ok(p) = parse {
            println!("Parse Successful");
            //p should be an array
            if let Value::Array(vec) = p {
                for item in vec {
                    //item should be an object
                    if let Value::Object(obj) = item {
                        let pmcid = obj.get("PMCID");
                        let pmcid = if let Some(file) = pmcid {
                            if let Value::String(s) = file {
                                if s.contains("PMCID: ") {
                                    Some(s.replace("PMCID: ", ""))
                                } else {
                                    println!("PMCID: not found :{:?}", s);
                                    None
                                }
                            } else {
                                println!("file name is not a string!: {:?}", file);
                                None
                            }
                        } else {
                            None
                        };
                        if pmcid.is_some()
                            && Path::new(&format!("./pdf/{}.pdf", pmcid.unwrap())).exists()
                        {
                            println!("File already downloaded");
                            continue;
                        }

                        let pdf_link = obj.get("pdf_link");
                        if pdf_link.is_none() {
                            println!("PDF link not found");
                            continue;
                        }

                        let pdf_link = pdf_link.unwrap();
                        if let Value::String(link) = pdf_link {
                            if !link.contains("https://") {
                                println!("Not link...");
                                continue;
                            }
                            println!("Getting: {}", link);
                            let response = client.get(link).send().await;
                            if response.is_err() {
                                println!("Get response is error!: {:?}", response);
                                continue;
                            }
                            let response = response.unwrap().bytes().await;
                            if response.is_err() {
                                println!("Get response Bytes is error!");
                                continue;
                            }
                            // println!("Response: {:?}", response);
                            let response = response.unwrap();
                            println!("Response size: {}", response.len());
                            let file_name = {
                                let file_name = obj.get("PMCID");
                                if let Some(file) = file_name {
                                    if let Value::String(s) = file {
                                        if s.contains("PMCID: ") {
                                            Some(s.replace("PMCID: ", ""))
                                        } else {
                                            println!("PMCID: not found :{:?}", s);
                                            None
                                        }
                                    } else {
                                        println!("file name is not a string!: {:?}", file);
                                        None
                                    }
                                } else {
                                    None
                                }
                            };
                            if file_name.is_none() {
                                println!("File name not found...");
                                continue;
                            }
                            let file_name = file_name.unwrap();
                            let tx_send_result = tx
                                .send(WriteRequest {
                                    file_name,
                                    bytes: response,
                                })
                                .await;
                            if tx_send_result.is_err() {
                                println!("Tx send errored!");
                                continue;
                            }
                        } else {
                            println!("pdf_link is not a String!: {:?}", pdf_link);
                            continue;
                        }
                    } else {
                        println!("Item is not obj!");
                        continue;
                    }
                }
            } else {
                println!("JSON is not vec!");
                continue;
            }
        } else {
            println!("Parse failed...");
            continue;
        }
        entries_read += 1;
    }
    println!("Read {} Entries!", entries_read);
    // println!("PMCID: PMC29183736");
    // println!("{:?}", "PMCID: PMC29183736".replace("PMCID: ", ""));
}

fn replace_invalid_characters(src: &str) -> String {
    let src = src.replace('<', "&lt;");
    let src = src.replace('&', "&amp;");
    let src = src.replace('\'', "&apos;");
    let src = src.replace('\"', "&quot;");

    src
}
