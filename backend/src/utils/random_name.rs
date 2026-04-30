use rand::Rng;

const ADJECTIVES: &[&str] = &[
    "快乐", "勇敢", "聪明", "可爱", "神秘", "优雅", "活泼", "温柔",
    "热情", "冷静", "幽默", "认真", "浪漫", "理性", "感性", "独立",
];

const NOUNS: &[&str] = &[
    "熊猫", "狐狸", "兔子", "松鼠", "海豚", "企鹅", "猫咪", "小狗",
    "考拉", "浣熊", "水獭", "刺猬", "仓鼠", "鹦鹉", "金鱼", "蝴蝶",
];

/// 生成随机游客昵称，格式：形容词+名词+4位数字
/// 例如：快乐熊猫4821
pub fn generate_visitor_name() -> String {
    let mut rng = rand::thread_rng();

    let adjective = ADJECTIVES[rng.gen_range(0..ADJECTIVES.len())];
    let noun = NOUNS[rng.gen_range(0..NOUNS.len())];
    let number = rng.gen_range(1000..10000);

    format!("{}{}{}", adjective, noun, number)
}
