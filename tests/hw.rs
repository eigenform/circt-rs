use circt_rs;
use melior::ir::attribute::{ArrayAttribute, StringAttribute, TypeAttribute};
use melior::ir::operation::OperationPrintingFlags;
use melior::ir::{Block, Location, Region, Type};
use melior::Context;

#[test]
fn create_hw_module() {
    let context = Context::new();
    context.set_allow_unregistered_dialects(true);

    let loc = Location::unknown(&context);

    let body_block = Block::new(&[]);
    let body_region = Region::new();
    body_region.append_block(body_block);

    let sym_name = StringAttribute::new(&context, "test");
    let module_type = TypeAttribute::new(Type::parse(&context, "!hw.module_type<>").unwrap());
    let parameters = ArrayAttribute::new(&context, &[]);

    let module = circt_rs::hw::module(
        &context,
        body_region,
        sym_name,
        module_type,
        parameters,
        loc,
    );

    let flags = OperationPrintingFlags::default();
    let text = module.as_operation().to_string_with_flags(flags).unwrap();

    assert_eq!(
        text,
        "\"hw.module\"() ({\n^bb0:\n}) {module_type = !hw.module_type<>, parameters = [], sym_name = \"test\"} : () -> ()\n"
    );
}
