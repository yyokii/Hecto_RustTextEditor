#[derive(PartialEq, Clone, Copy)]
pub enum Type {
    None,
    Number,
    Match,
    String,
    Character,
    Comment,
    MultilineComment,
    PrimaryKeywords,
    SecondaryKeywords,
}

impl  Type {
    /*
    ANSIエスケープコード: https://www.mm2d.net/main/prog/c/console-02.html
    ANSIエスケープシーケンス チートシート: https://qiita.com/PruneMazui/items/8a023347772620025ad6
    */
    pub fn to_color_sequence(self) -> &'static str {
        match self {
            Type::Match => "\x1b[36m",
            Type::Number => "\x1b[35m",
            Type::String => "\x1b[31m",
            Type::Character => "\x1b[34m",
            Type::Comment | Type::MultilineComment => "\x1b[32m",
            Type::PrimaryKeywords => "\x1b[33m",
            Type::SecondaryKeywords => "\x1b[92m",
            _ => "\x1b[90m",
        }
    }
}