use darling::FromDeriveInput;
use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};


#[derive(FromDeriveInput, Default)]
#[darling(default, attributes(data_value))]
struct DataValueOptions {
    data_path: String,
}

#[proc_macro_derive(DataValue, attributes(data_value))]
pub fn dv_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);
    let opts = DataValueOptions::from_derive_input(&input).expect("Wrong options");
    let DeriveInput { ident, .. } = input;
    let data_path = opts.data_path;
    let answer = quote! {
        fn name(&self) -> String {
            self.name.to_string()
        }

        fn unit_name(&self) -> String {
            self.display_unit.abbreviation()
        }

        fn abbreviation(&self) -> String {
            self.abbreviation.to_string()
        }

        fn add_config(&mut self, index: usize, ui: &mut Ui) {
            self.display_unit.add_config(index, ui);
        }

        fn fmt_value(&self, communicator: &SignalKCommunicator) -> String {
            let temp =
                communicator.get_f64_for_path(#data_path.to_string());
            self.display_unit.format(temp)
        }
    };

    let output = quote! {
        impl DataValue for #ident {
            #answer
        }
    };
    output.into()
}

