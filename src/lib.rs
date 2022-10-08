pub mod basic;
pub mod rv32i;

#[cfg(test)]
mod tests {
    use std::{fs::File, path::Path};

    use faerie::{ArtifactBuilder, Decl};
    use std::str::FromStr;
    use target_lexicon::triple;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn elf_binary_test() {
        let name = "test";
        let file = File::create(Path::new(name)).unwrap();
        let mut obj = ArtifactBuilder::new(triple!("riscv32-unknown-elf"))
            .name(name.to_owned())
            .finish();
        obj.declarations(
            [("main", Decl::function().global().into())].iter().cloned(),
        )
        .unwrap();
        obj.define("main", vec![0x0]).unwrap();
        obj.write(file).unwrap();
    }
}
