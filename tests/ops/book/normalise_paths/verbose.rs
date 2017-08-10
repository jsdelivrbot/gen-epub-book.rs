use gen_epub_book::ops::{EPubContentType, BookElement, EPubBook};
use self::super::make_test_folder;
use std::fs::{self, File};
use gen_epub_book::Error;
use std::path::PathBuf;
use chrono::DateTime;
use std::str;


#[test]
fn correct() {
    let tf = make_test_folder("verbose-correct");
    let _ = fs::create_dir(tf.join("content"));
    File::create(tf.join("cover.png")).unwrap();
    File::create(tf.join("content").join("ch01.html")).unwrap();

    let mut buf = vec![];
    let mut book = EPubBook::from_elements(vec![BookElement::Name("".to_string()),
                                                BookElement::Author("".to_string()),
                                                BookElement::Date(DateTime::parse_from_rfc3339("2017-02-08T15:30:18+01:00").unwrap()),
                                                BookElement::Language("".to_string()),
                                                BookElement::Cover(PathBuf::from("cover.png")),
                                                BookElement::Content(PathBuf::from("content/ch01.html"))])
        .unwrap();

    assert_eq!(book.normalise_paths(&("$TEMP/ops-book-normalise-paths-verbose-correct/".to_string(), tf.clone()), true, &mut buf),
               Ok(()));
    assert_eq!(book.cover,
               Some(("cover-content-4".to_string(),
                     PathBuf::from("cover-data-4.html"),
                     EPubContentType::Raw(r#"<center><img src="cover.png" alt="cover.png"></img></center>"#.to_string()))));
    assert_eq!(str::from_utf8(&buf).unwrap(),
               "Normalised content/ch01.html to $TEMP/ops-book-normalise-paths-verbose-correct/content/ch01.html for Content, Image or Include.\n\
                Normalised cover.png to $TEMP/ops-book-normalise-paths-verbose-correct/cover.png for Content, Image or Include.\n");
}

#[test]
fn nonexistant() {
    let tf = make_test_folder("verbose-nonexistant");
    File::create(tf.join("cover.png")).unwrap();

    let mut buf = vec![];
    let mut book = EPubBook::from_elements(vec![BookElement::Name("".to_string()),
                                                BookElement::Author("".to_string()),
                                                BookElement::Date(DateTime::parse_from_rfc3339("2017-02-08T15:30:18+01:00").unwrap()),
                                                BookElement::Language("".to_string()),
                                                BookElement::Content(PathBuf::from("ch01.html"))])
        .unwrap();

    assert_eq!(book.normalise_paths(&("$TEMP/ops-book-normalise-paths-verbose-nonexistant/".to_string(), tf.clone()), true, &mut buf),
               Err(Error::FileNotFound {
                   who: "Content, Image or Include",
                   path: tf.join("ch01.html"),
               }));
    assert!(buf.is_empty());
}

#[test]
fn bad_type() {
    let tf = make_test_folder("verbose-bad-type");
    let _ = fs::create_dir(tf.join("ch01.html"));
    File::create(tf.join("cover.png")).unwrap();

    let mut buf = vec![];
    let mut book = EPubBook::from_elements(vec![BookElement::Name("".to_string()),
                                                BookElement::Author("".to_string()),
                                                BookElement::Date(DateTime::parse_from_rfc3339("2017-02-08T15:30:18+01:00").unwrap()),
                                                BookElement::Language("".to_string()),
                                                BookElement::Cover(PathBuf::from("cover.png")),
                                                BookElement::Content(PathBuf::from("ch01.html"))])
        .unwrap();

    assert_eq!(book.normalise_paths(&("$TEMP/ops-book-normalise-paths-verbose-bad-type/".to_string(), tf.clone()), true, &mut buf),
               Err(Error::WrongFileState {
                   what: "a file",
                   path: tf.join("ch01.html"),
               }));
    assert!(buf.is_empty());
}
