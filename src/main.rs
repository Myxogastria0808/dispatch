pub trait ShowPointInterface {
    fn show(&self);
}

pub struct FieldPoint {
    pub x: isize,
    pub y: isize,
}

impl ShowPointInterface for FieldPoint {
    fn show(&self) {
        println!("Point({}, {})", self.x, self.y);
    }
}

pub struct SpacePoint {
    pub x: isize,
    pub y: isize,
    pub z: isize,
}

impl ShowPointInterface for SpacePoint {
    fn show(&self) {
        println!("Point({}, {}, {})", self.x, self.y, self.z);
    }
}

// Rustはコンパイル時に「単態化（monomorphization）」を行うため、
// static_show_point関数は、呼び出し時に具体的な型に置き換えられる。
// 参考サイト: https://qiita.com/Leapcell/items/805a6f7af4d03c8bb6df
fn static_show_point<T: ShowPointInterface>(point: &T) {
    point.show();
}

// 以下のように記述する方法も考えられる (上記の糖衣構文にあたる)
// 参考サイト: https://zenn.dev/woden/articles/9787e533d05e0f
// &impl を使用すると、静的ディスパッチが行われる
fn static_show_point2(point: &impl ShowPointInterface) {
    point.show();
}

// &dyn を使用すると、動的ディスパッチが行われる
fn dynamic_show_point(point: &dyn ShowPointInterface) {
    point.show();
}

fn main() {
    let field_point = FieldPoint { x: 1, y: 2 };
    let space_point = SpacePoint { x: 3, y: 4, z: 5 };

    // 静的ディスパッチ
    static_show_point(&field_point);
    static_show_point(&space_point);

    // 静的ディスパッチの別の書き方
    static_show_point2(&field_point);
    static_show_point2(&space_point);

    // 動的ディスパッチ
    dynamic_show_point(&field_point);
    dynamic_show_point(&space_point);
}
