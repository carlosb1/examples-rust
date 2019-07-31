struct InfoValue<V> {
    value: V,
    info: &'static str,
}

pub trait Info {
    fn get_info(&self) -> &'static str;
}

impl Info for InfoValue<u32>  {
    fn get_info(&self) -> &'static str {
        self.info
    }
}

struct ImprovedPoint<T: Info> {
    x: T,
    y: T,
}

struct ContainerEx {
    improved_integer: ImprovedPoint<InfoValue<u32>>
}


fn main() {
    let impr_x = InfoValue{value: 5, info: "is a 5"};
    let impr_y = InfoValue{value: 10, info: "is a 10"};
    let improved_integer = ImprovedPoint{x: impr_x, y: impr_y};
    let container = ContainerEx{improved_integer: improved_integer};

}
