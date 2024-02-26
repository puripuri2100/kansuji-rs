//! # 概要
//!
//! 漢数字の解析と変換を行うcrateである。
//! サポートする漢数字の桁の範囲は垓(10^20)から毛(10^-3)までとする
//! (<https://homepage45.net/unit/sub.htm>)
//!
//! なお、大字をどこまでサポートするかは今後決めるものとする。
//!

use std::convert::{From, TryFrom};
use std::string::String;
use thiserror::Error;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum KansujiField {
    零,
    一,
    二,
    三,
    四,
    五,
    六,
    七,
    八,
    九,
}

impl KansujiField {
    fn from_int(n: u8) -> Self {
        match n {
            0 => KansujiField::零,
            1 => KansujiField::一,
            2 => KansujiField::二,
            3 => KansujiField::三,
            4 => KansujiField::四,
            5 => KansujiField::五,
            6 => KansujiField::六,
            7 => KansujiField::七,
            8 => KansujiField::八,
            9 => KansujiField::九,
            _ => unreachable!(),
        }
    }

    fn to_int(self) -> u8 {
        match self {
            KansujiField::零 => 0,
            KansujiField::一 => 1,
            KansujiField::二 => 2,
            KansujiField::三 => 3,
            KansujiField::四 => 4,
            KansujiField::五 => 5,
            KansujiField::六 => 6,
            KansujiField::七 => 7,
            KansujiField::八 => 8,
            KansujiField::九 => 9,
        }
    }

    fn to_str(self) -> String {
        match self {
            KansujiField::零 => String::new(),
            KansujiField::一 => String::new(),
            KansujiField::二 => "二".to_string(),
            KansujiField::三 => "三".to_string(),
            KansujiField::四 => "四".to_string(),
            KansujiField::五 => "五".to_string(),
            KansujiField::六 => "六".to_string(),
            KansujiField::七 => "七".to_string(),
            KansujiField::八 => "八".to_string(),
            KansujiField::九 => "九".to_string(),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct KansujiKeta {
    千: KansujiField,
    百: KansujiField,
    十: KansujiField,
    一: KansujiField,
}

impl KansujiKeta {
    fn is_zero(self) -> bool {
        self.千 == KansujiField::零
            && self.百 == KansujiField::零
            && self.十 == KansujiField::零
            && self.一 == KansujiField::零
    }
    fn is_one(self) -> bool {
        self.千 == KansujiField::零
            && self.百 == KansujiField::零
            && self.十 == KansujiField::零
            && self.一 == KansujiField::一
    }
}

impl ToString for KansujiKeta {
    fn to_string(&self) -> String {
        let mut s = String::new();
        if self.千 != KansujiField::零 {
            s.push_str(&format!("{}千", self.千.to_str()))
        }
        if self.百 != KansujiField::零 {
            s.push_str(&format!("{}百", self.百.to_str()))
        }
        if self.十 != KansujiField::零 {
            s.push_str(&format!("{}十", self.十.to_str()))
        }
        s.push_str(&self.一.to_str());
        s
    }
}

impl From<KansujiKeta> for usize {
    fn from(value: KansujiKeta) -> Self {
        let mut n = value.一.to_int() as usize;
        n += value.十.to_int() as usize * 10;
        n += value.百.to_int() as usize * 100;
        n += value.千.to_int() as usize * 1000;
        n
    }
}

impl From<usize> for KansujiKeta {
    fn from(value: usize) -> Self {
        let n = value % 10000;
        let sen = (n / 1000) as u8;
        let hyaku = ((n % 1000) / 100) as u8;
        let juu = ((n % 100) / 10) as u8;
        let ichi = (n % 10) as u8;
        KansujiKeta {
            千: KansujiField::from_int(sen),
            百: KansujiField::from_int(hyaku),
            十: KansujiField::from_int(juu),
            一: KansujiField::from_int(ichi),
        }
    }
}

impl Default for KansujiKeta {
    fn default() -> Self {
        KansujiKeta {
            千: KansujiField::零,
            百: KansujiField::零,
            十: KansujiField::零,
            一: KansujiField::零,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Kansuji {
    垓: KansujiKeta,
    京: KansujiKeta,
    兆: KansujiKeta,
    億: KansujiKeta,
    万: KansujiKeta,
    一: KansujiKeta,
    分: KansujiField,
    厘: KansujiField,
    毛: KansujiField,
}

impl Default for Kansuji {
    fn default() -> Self {
        Kansuji {
            垓: KansujiKeta::default(),
            京: KansujiKeta::default(),
            兆: KansujiKeta::default(),
            億: KansujiKeta::default(),
            万: KansujiKeta::default(),
            一: KansujiKeta::default(),
            分: KansujiField::零,
            厘: KansujiField::零,
            毛: KansujiField::零,
        }
    }
}

#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum KansujiError {
    #[error("parse error")]
    ParseError,
    #[error("unexpected char: {0}")]
    UnexpectedChar(char),
    #[error("unexpected end")]
    UnexpectedEnd,
    #[error("too large")]
    TooLarge,
}

impl TryFrom<String> for Kansuji {
    type Error = KansujiError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let chars = value.chars();
        parse_kansuji(chars)
    }
}

impl TryFrom<&String> for Kansuji {
    type Error = KansujiError;
    fn try_from(value: &String) -> Result<Self, Self::Error> {
        let chars = value.chars();
        parse_kansuji(chars)
    }
}

impl TryFrom<&str> for Kansuji {
    type Error = KansujiError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let chars = value.chars();
        parse_kansuji(chars)
    }
}

fn parse_kansuji(chars: std::str::Chars) -> Result<Kansuji, KansujiError> {
    let mut chars = chars.peekable();
    let mut kansuji = Kansuji::default();
    let mut keta = 6_i8;
    loop {
        let kansuji_keta = parse_keta(&mut chars)?;
        if let Some(c) = chars.peek() {
            match c {
                '垓' => {
                    if keta > 5 {
                        kansuji = Kansuji {
                            垓: kansuji_keta,
                            ..kansuji
                        };
                        chars.next();
                        keta = 5;
                    } else {
                        return Err(KansujiError::UnexpectedChar(*c));
                    }
                }
                '京' => {
                    if keta > 4 {
                        kansuji = Kansuji {
                            京: kansuji_keta,
                            ..kansuji
                        };
                        chars.next();
                        keta = 4;
                    } else {
                        return Err(KansujiError::UnexpectedChar(*c));
                    }
                }
                '兆' => {
                    if keta > 3 {
                        kansuji = Kansuji {
                            兆: kansuji_keta,
                            ..kansuji
                        };
                        chars.next();
                        keta = 3;
                    } else {
                        return Err(KansujiError::UnexpectedChar(*c));
                    }
                }
                '億' => {
                    if keta > 2 {
                        kansuji = Kansuji {
                            億: kansuji_keta,
                            ..kansuji
                        };
                        chars.next();
                        keta = 2;
                    } else {
                        return Err(KansujiError::UnexpectedChar(*c));
                    }
                }
                '万' => {
                    if keta > 1 {
                        kansuji = Kansuji {
                            万: kansuji_keta,
                            ..kansuji
                        };
                        chars.next();
                        keta = 1;
                    } else {
                        return Err(KansujiError::UnexpectedChar(*c));
                    }
                }
                '分' => {
                    if keta > -1
                        && kansuji_keta.百 == KansujiField::零
                        && kansuji_keta.千 == KansujiField::零
                        && kansuji_keta.十 == KansujiField::零
                    {
                        kansuji = Kansuji {
                            分: kansuji_keta.一,
                            ..kansuji
                        };
                        chars.next();
                        keta = -1;
                    } else {
                        return Err(KansujiError::UnexpectedChar(*c));
                    }
                }
                '厘' => {
                    if keta > -2
                        && kansuji_keta.百 == KansujiField::零
                        && kansuji_keta.千 == KansujiField::零
                        && kansuji_keta.十 == KansujiField::零
                    {
                        kansuji = Kansuji {
                            厘: kansuji_keta.一,
                            ..kansuji
                        };
                        chars.next();
                        keta = -2;
                    } else {
                        return Err(KansujiError::UnexpectedChar(*c));
                    }
                }
                '毛' => {
                    if keta > -3
                        && kansuji_keta.百 == KansujiField::零
                        && kansuji_keta.千 == KansujiField::零
                        && kansuji_keta.十 == KansujiField::零
                    {
                        kansuji = Kansuji {
                            毛: kansuji_keta.一,
                            ..kansuji
                        };
                        chars.next();
                        break;
                    } else {
                        return Err(KansujiError::UnexpectedChar(*c));
                    }
                }
                _ => {
                    if keta > 0 {
                        kansuji = Kansuji {
                            一: kansuji_keta,
                            ..kansuji
                        };
                        chars.next();
                        keta = 0;
                    } else {
                        break;
                    }
                }
            }
        } else {
            kansuji = Kansuji {
                一: kansuji_keta,
                ..kansuji
            };
            break;
        }
    }
    Ok(kansuji)
}

#[test]
fn check_parse_kansuji_1() {
    let str = "百万一";
    let kansuji = parse_kansuji(str.chars());
    assert_eq!(
        kansuji,
        Ok(Kansuji {
            垓: KansujiKeta::default(),
            京: KansujiKeta::default(),
            兆: KansujiKeta::default(),
            億: KansujiKeta::default(),
            万: KansujiKeta {
                百: KansujiField::一,
                ..KansujiKeta::default()
            },
            一: KansujiKeta {
                一: KansujiField::一,
                ..KansujiKeta::default()
            },
            分: KansujiField::零,
            厘: KansujiField::零,
            毛: KansujiField::零,
        })
    )
}

#[test]
fn check_parse_kansuji_2() {
    let str = "二百五垓百万二十一";
    let kansuji = parse_kansuji(str.chars());
    assert_eq!(
        kansuji,
        Ok(Kansuji {
            垓: KansujiKeta {
                百: KansujiField::二,
                一: KansujiField::五,
                ..KansujiKeta::default()
            },
            京: KansujiKeta::default(),
            兆: KansujiKeta::default(),
            億: KansujiKeta::default(),
            万: KansujiKeta {
                百: KansujiField::一,
                ..KansujiKeta::default()
            },
            一: KansujiKeta {
                十: KansujiField::二,
                一: KansujiField::一,
                ..KansujiKeta::default()
            },
            分: KansujiField::零,
            厘: KansujiField::零,
            毛: KansujiField::零,
        })
    )
}

#[test]
fn check_parse_kansuji_3() {
    let str = "二百五垓ほ百万二十一";
    let kansuji = parse_kansuji(str.chars());
    assert!(kansuji.is_err())
}

fn parse_keta(
    chars: &mut std::iter::Peekable<std::str::Chars>,
) -> Result<KansujiKeta, KansujiError> {
    let mut sen = None;
    let mut hyaku = None;
    let mut juu = None;
    let mut iti = None;
    let mut keta = 4_u8;
    let mut field = None;
    while keta > 0 {
        if let Some(c) = chars.peek() {
            match c {
                '一' => {
                    field = Some(KansujiField::一);
                    chars.next();
                }
                '二' => {
                    field = Some(KansujiField::二);
                    chars.next();
                }
                '三' => {
                    field = Some(KansujiField::三);
                    chars.next();
                }
                '四' => {
                    field = Some(KansujiField::四);
                    chars.next();
                }
                '五' => {
                    field = Some(KansujiField::五);
                    chars.next();
                }
                '六' => {
                    field = Some(KansujiField::六);
                    chars.next();
                }
                '七' => {
                    field = Some(KansujiField::七);
                    chars.next();
                }
                '八' => {
                    field = Some(KansujiField::八);
                    chars.next();
                }
                '九' => {
                    field = Some(KansujiField::九);
                    chars.next();
                }
                '千' => {
                    if keta > 3 {
                        if let Some(f) = field {
                            sen = Some(f)
                        } else {
                            sen = Some(KansujiField::一)
                        }
                        chars.next();
                        keta = 3;
                        field = None;
                    } else {
                        return Err(KansujiError::UnexpectedChar(*c));
                    }
                }
                '百' => {
                    if keta > 2 {
                        if let Some(f) = field {
                            hyaku = Some(f)
                        } else {
                            hyaku = Some(KansujiField::一)
                        }
                        chars.next();
                        keta = 2;
                        field = None;
                    } else {
                        return Err(KansujiError::UnexpectedChar(*c));
                    }
                }
                '十' => {
                    if keta > 1 {
                        if let Some(f) = field {
                            juu = Some(f)
                        } else {
                            juu = Some(KansujiField::一)
                        }
                        chars.next();
                        keta = 1;
                        field = None;
                    } else {
                        return Err(KansujiError::UnexpectedChar(*c));
                    }
                }
                '万' | '兆' | '京' | '垓' => {
                    if let Some(f) = field {
                        iti = Some(f);
                    } else {
                        iti = Some(KansujiField::零);
                    }
                    break;
                }
                c => return Err(KansujiError::UnexpectedChar(*c)),
            }
        } else {
            if let Some(f) = field {
                iti = Some(f);
            } else {
                iti = Some(KansujiField::零);
            }
            break;
        }
    }
    Ok(KansujiKeta {
        千: sen.unwrap_or(KansujiField::零),
        百: hyaku.unwrap_or(KansujiField::零),
        十: juu.unwrap_or(KansujiField::零),
        一: iti.unwrap_or(KansujiField::零),
    })
}

#[test]
fn check_parse_keta_1() {
    let mut chars = "百三十一".chars().peekable();
    let keta = parse_keta(&mut chars);
    assert_eq!(
        keta,
        Ok(KansujiKeta {
            千: KansujiField::零,
            百: KansujiField::一,
            十: KansujiField::三,
            一: KansujiField::一
        })
    )
}

#[test]
fn check_parse_keta_2() {
    let mut chars = "".chars().peekable();
    let keta = parse_keta(&mut chars);
    assert_eq!(
        keta,
        Ok(KansujiKeta {
            千: KansujiField::零,
            百: KansujiField::零,
            十: KansujiField::零,
            一: KansujiField::零
        })
    )
}

#[test]
fn check_parse_keta_3() {
    let mut chars = "百万一".chars().peekable();
    let keta = parse_keta(&mut chars);
    assert_eq!(
        keta,
        Ok(KansujiKeta {
            千: KansujiField::零,
            百: KansujiField::一,
            十: KansujiField::零,
            一: KansujiField::零
        })
    );
    chars.next();
    let keta = parse_keta(&mut chars);
    assert_eq!(
        keta,
        Ok(KansujiKeta {
            千: KansujiField::零,
            百: KansujiField::零,
            十: KansujiField::零,
            一: KansujiField::一
        })
    )
}

#[test]
fn check_parse_keta_4() {
    let mut chars = "百に万一".chars().peekable();
    let keta = parse_keta(&mut chars);
    assert!(keta.is_err())
}

#[test]
fn check_parse_keta_5() {
    let mut chars = "百二万一".chars().peekable();
    let keta = parse_keta(&mut chars);
    assert_eq!(
        keta,
        Ok(KansujiKeta {
            千: KansujiField::零,
            百: KansujiField::一,
            十: KansujiField::零,
            一: KansujiField::二
        })
    );
    chars.next();
    let keta = parse_keta(&mut chars);
    assert_eq!(
        keta,
        Ok(KansujiKeta {
            千: KansujiField::零,
            百: KansujiField::零,
            十: KansujiField::零,
            一: KansujiField::一
        })
    )
}

impl From<Kansuji> for f64 {
    fn from(value: Kansuji) -> Self {
        let mut n = 0;
        n += Into::<usize>::into(value.一) as u128;
        n += (Into::<usize>::into(value.万) as u128) * 10000;
        n += (Into::<usize>::into(value.億) as u128) * 100000000;
        n += (Into::<usize>::into(value.兆) as u128) * 1000000000000;
        n += (Into::<usize>::into(value.京) as u128) * 10000000000000000;
        n += (Into::<usize>::into(value.垓) as u128) * 100000000000000000000;
        let mut n2 = 0;
        n2 += value.分.to_int() as usize * 100;
        n2 += value.厘.to_int() as usize * 10;
        n2 += value.毛.to_int() as usize;
        n as f64 + (n2 as f64 * 0.001)
    }
}

impl From<Kansuji> for f32 {
    fn from(value: Kansuji) -> Self {
        let mut n = 0;
        n += Into::<usize>::into(value.一) as u128;
        n += (Into::<usize>::into(value.万) as u128) * 10000;
        n += (Into::<usize>::into(value.億) as u128) * 100000000;
        n += (Into::<usize>::into(value.兆) as u128) * 1000000000000;
        n += (Into::<usize>::into(value.京) as u128) * 10000000000000000;
        n += (Into::<usize>::into(value.垓) as u128) * 100000000000000000000;
        let mut n2 = 0;
        n2 += value.分.to_int() as usize * 100;
        n2 += value.厘.to_int() as usize * 10;
        n2 += value.毛.to_int() as usize;
        n as f32 + (n2 as f32 * 0.001)
    }
}

impl From<Kansuji> for u128 {
    fn from(value: Kansuji) -> Self {
        let mut n = 0;
        n += Into::<usize>::into(value.一) as u128;
        n += (Into::<usize>::into(value.万) as u128) * 10000;
        n += (Into::<usize>::into(value.億) as u128) * 100000000;
        n += (Into::<usize>::into(value.兆) as u128) * 1000000000000;
        n += (Into::<usize>::into(value.京) as u128) * 10000000000000000;
        n += (Into::<usize>::into(value.垓) as u128) * 100000000000000000000;
        n
    }
}

impl From<u128> for Kansuji {
    fn from(value: u128) -> Self {
        let gai = value / 100000000000000000000;
        let kei = (value % 100000000000000000000) / 10000000000000000;
        let tyou = (value % 10000000000000000) / 1000000000000;
        let oku = (value % 1000000000000) / 100000000;
        let man = (value % 100000000) / 10000;
        let iti = value % 10000;
        Kansuji {
            垓: KansujiKeta::from(gai as usize),
            京: KansujiKeta::from(kei as usize),
            兆: KansujiKeta::from(tyou as usize),
            億: KansujiKeta::from(oku as usize),
            万: KansujiKeta::from(man as usize),
            一: KansujiKeta::from(iti as usize),
            分: KansujiField::零,
            厘: KansujiField::零,
            毛: KansujiField::零,
        }
    }
}

impl From<usize> for Kansuji {
    fn from(value: usize) -> Self {
        let v = value as u64;
        Kansuji::from(v)
    }
}

impl From<u64> for Kansuji {
    fn from(value: u64) -> Self {
        let kei = value / 10000000000000000;
        let tyou = (value % 10000000000000000) / 1000000000000;
        let oku = (value % 1000000000000) / 100000000;
        let man = (value % 100000000) / 10000;
        let iti = value % 10000;
        Kansuji {
            垓: KansujiKeta::default(),
            京: KansujiKeta::from(kei as usize),
            兆: KansujiKeta::from(tyou as usize),
            億: KansujiKeta::from(oku as usize),
            万: KansujiKeta::from(man as usize),
            一: KansujiKeta::from(iti as usize),
            分: KansujiField::零,
            厘: KansujiField::零,
            毛: KansujiField::零,
        }
    }
}

impl From<u32> for Kansuji {
    fn from(value: u32) -> Self {
        let oku = value / 100000000;
        let man = (value % 100000000) / 10000;
        let iti = value % 10000;
        Kansuji {
            垓: KansujiKeta::default(),
            京: KansujiKeta::default(),
            兆: KansujiKeta::default(),
            億: KansujiKeta::from(oku as usize),
            万: KansujiKeta::from(man as usize),
            一: KansujiKeta::from(iti as usize),
            分: KansujiField::零,
            厘: KansujiField::零,
            毛: KansujiField::零,
        }
    }
}

impl From<u16> for Kansuji {
    fn from(value: u16) -> Self {
        let man = value / 10000;
        let iti = value % 10000;
        Kansuji {
            垓: KansujiKeta::default(),
            京: KansujiKeta::default(),
            兆: KansujiKeta::default(),
            億: KansujiKeta::default(),
            万: KansujiKeta::from(man as usize),
            一: KansujiKeta::from(iti as usize),
            分: KansujiField::零,
            厘: KansujiField::零,
            毛: KansujiField::零,
        }
    }
}

impl From<u8> for Kansuji {
    fn from(value: u8) -> Self {
        Kansuji {
            垓: KansujiKeta::default(),
            京: KansujiKeta::default(),
            兆: KansujiKeta::default(),
            億: KansujiKeta::default(),
            万: KansujiKeta::default(),
            一: KansujiKeta::from(value as usize),
            分: KansujiField::零,
            厘: KansujiField::零,
            毛: KansujiField::零,
        }
    }
}

impl From<f64> for Kansuji {
    fn from(value: f64) -> Self {
        let n = value as u128;
        let gai = n / 100000000000000000000;
        let kei = (n % 100000000000000000000) / 10000000000000000;
        let tyou = (n % 10000000000000000) / 1000000000000;
        let oku = (n % 1000000000000) / 100000000;
        let man = (n % 100000000) / 10000;
        let iti = n % 10000;
        let f = value - (n as f64);
        let f = (f * 1000.0) as usize;
        let bu = f / 100;
        let rin = (f % 100) / 10;
        let mou = f % 10;
        Kansuji {
            垓: KansujiKeta::from(gai as usize),
            京: KansujiKeta::from(kei as usize),
            兆: KansujiKeta::from(tyou as usize),
            億: KansujiKeta::from(oku as usize),
            万: KansujiKeta::from(man as usize),
            一: KansujiKeta::from(iti as usize),
            分: KansujiField::from_int(bu as u8),
            厘: KansujiField::from_int(rin as u8),
            毛: KansujiField::from_int(mou as u8),
        }
    }
}

impl From<f32> for Kansuji {
    fn from(value: f32) -> Self {
        let n = value as u128;
        let gai = n / 100000000000000000000;
        let kei = (n % 100000000000000000000) / 10000000000000000;
        let tyou = (n % 10000000000000000) / 1000000000000;
        let oku = (n % 1000000000000) / 100000000;
        let man = (n % 100000000) / 10000;
        let iti = n % 10000;
        let f = value - (n as f32);
        let f = (f * 1000.0) as usize;
        let bu = f / 100;
        let rin = (f % 100) / 10;
        let mou = f % 10;
        Kansuji {
            垓: KansujiKeta::from(gai as usize),
            京: KansujiKeta::from(kei as usize),
            兆: KansujiKeta::from(tyou as usize),
            億: KansujiKeta::from(oku as usize),
            万: KansujiKeta::from(man as usize),
            一: KansujiKeta::from(iti as usize),
            分: KansujiField::from_int(bu as u8),
            厘: KansujiField::from_int(rin as u8),
            毛: KansujiField::from_int(mou as u8),
        }
    }
}

impl From<&u128> for Kansuji {
    fn from(value: &u128) -> Self {
        Kansuji::from(*value)
    }
}

impl From<&u64> for Kansuji {
    fn from(value: &u64) -> Self {
        Kansuji::from(*value)
    }
}

impl From<&u32> for Kansuji {
    fn from(value: &u32) -> Self {
        Kansuji::from(*value)
    }
}

impl From<&u16> for Kansuji {
    fn from(value: &u16) -> Self {
        Kansuji::from(*value)
    }
}

impl From<&u8> for Kansuji {
    fn from(value: &u8) -> Self {
        Kansuji::from(*value)
    }
}

impl From<&usize> for Kansuji {
    fn from(value: &usize) -> Self {
        Kansuji::from(*value)
    }
}

impl From<&f64> for Kansuji {
    fn from(value: &f64) -> Self {
        Kansuji::from(*value)
    }
}

impl From<&f32> for Kansuji {
    fn from(value: &f32) -> Self {
        Kansuji::from(*value)
    }
}

impl ToString for Kansuji {
    fn to_string(&self) -> String {
        let mut s = String::new();
        if self.垓.is_zero()
            && self.京.is_zero()
            && self.兆.is_zero()
            && self.億.is_zero()
            && self.万.is_zero()
            && self.一.is_zero()
            && self.分 == KansujiField::零
            && self.厘 == KansujiField::零
            && self.毛 == KansujiField::零
        {
            return "零".to_string();
        }
        if !self.垓.is_zero() {
            s.push_str(&format!("{}垓", self.垓.to_string()))
        }
        if !self.京.is_zero() {
            s.push_str(&format!("{}京", self.京.to_string()))
        }
        if !self.兆.is_zero() {
            s.push_str(&format!("{}兆", self.兆.to_string()))
        }
        if !self.億.is_zero() {
            s.push_str(&format!("{}億", self.億.to_string()))
        }
        if !self.万.is_zero() {
            s.push_str(&format!("{}万", self.万.to_string()))
        }
        if self.一.is_one() {
            s.push('一')
        } else {
            s.push_str(&self.一.to_string())
        }
        if self.分 != KansujiField::零 {
            s.push_str(&format!("{}分", self.分.to_str()))
        }
        if self.厘 != KansujiField::零 {
            s.push_str(&format!("{}厘", self.厘.to_str()))
        }
        if self.毛 != KansujiField::零 {
            s.push_str(&format!("{}毛", self.毛.to_str()))
        }
        s
    }
}

#[test]
fn check_kansuji_1() {
    fn kansuji_test_function(n: &u128) {
        let kansuji = Kansuji::from(n);
        assert_eq!(*n, kansuji.into());
        let s = kansuji.to_string();
        let new_kansuji = Kansuji::try_from(&s);
        assert_eq!(new_kansuji, Ok(kansuji));
        assert_eq!(s, new_kansuji.unwrap().to_string());
    }

    let v = vec![0, 1, 2, 3, 10, 11, 15, 200, 76492334, 764923341, 1999999];
    let _ = v.iter().map(kansuji_test_function);
}

#[test]
fn check_kansuji_2() {
    let f = 1.234;
    let kansuji = Kansuji::from(f);
    let s = kansuji.to_string();
    assert_eq!(s, "一二分三厘四毛".to_string());
}

#[test]
fn check_kansuji_3() {
    let f = 1.203;
    let kansuji = Kansuji::from(f);
    let s = kansuji.to_string();
    assert_eq!(s, "一二分三毛".to_string());
}
