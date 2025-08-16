use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct ParseVersionError;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Version {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
}

impl Version {
    pub fn new(major: u64, minor: u64, patch: u64) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    pub fn to_string(&self) -> String {
        format!("{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl FromStr for Version {
    // エラー型として上で定義したものを指定
    type Err = ParseVersionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // 1. 文字列を"."で分割する
        let parts: Vec<&str> = s.split('.').collect();

        // 2. 要素が3つでなければエラー
        if parts.len() != 3 {
            return Err(ParseVersionError);
        }

        // 3. 各要素をu64にパースする
        //    どれか一つでもパースに失敗したらエラー
        let major = parts[0].parse::<u64>().map_err(|_| ParseVersionError)?;
        let minor = parts[1].parse::<u64>().map_err(|_| ParseVersionError)?;
        let patch = parts[2].parse::<u64>().map_err(|_| ParseVersionError)?;

        // 4. パースした値でVersion構造体を作成して返す
        Ok(Version {
            major,
            minor,
            patch,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_string_test() {
        let version = Version::new(1, 2, 3);
        assert_eq!(version.to_string(), "1.2.3");
    }

    #[test]
    fn test_parse_valid_version() {
        //      .parse()メソッドを呼び出すと、内部でFromStr::from_str      が呼ばれる
        let version: Version = "1.2.3".parse().unwrap();
        assert_eq!(
            version,
            Version {
                major: 1,
                minor: 2,
                patch: 3
            }
        );
    }

    #[test]
    fn test_parse_invalid_version() {
        //      不正な文字列をパースするとErrが返ることを確認
        let result: Result<Version, _> = "1.2.a".parse();
        assert_eq!(result, Err(ParseVersionError));
    }
}
