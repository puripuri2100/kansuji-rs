//! 垓から毛までをサポートすることにする
//! <https://homepage45.net/unit/sub.htm>
//! 大字をどこまでサポートするかは今後決める

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
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct KansujiKeta {
    千: KansujiField,
    百: KansujiField,
    十: KansujiField,
    一: KansujiField,
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



