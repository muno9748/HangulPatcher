#[derive(PartialEq, Eq, Debug)]
pub enum HistoryType {
    Null, // Dummy value for initializing

    A, // ex) ㄱ
    B, // ex) ㅏ
    Cc, // ex) ㄳ
    Bb, // ex) ㅢ
    Ab, // ex) 가
    Abb, // ex) 긔
    Abc, // ex) 각
    Abbc, // ex) 긕
    Abcc, // ex) 갃
    Abbcc, // ex) 긗
}