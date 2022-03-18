# Franca IDL

An incomplete Rust Franca IDL parser crate based on nom.

# Reference

- [Franca Google Drive](https://drive.google.com/drive/folders/0B7JseVbR6jvhUnhLOUM5ZGxOOG8?resourcekey=0-U-X53hicOvlqAZCG86dCUQ)

# Why Rust, Why not existing solutions

One goal of this library is to allow generating Rust SomeIP bindings and clients similar to capicxx-someip-tools.
This is certainly possible using Xtend and the existing generator, obviously this provides the advantage that all Franca parsing is already taken care of.
This approach was also chosen in [Rust-based SOME/IP implementation for robust automotive software](https://repositorio-aberto.up.pt/bitstream/10216/133192/2/449878.pdf) for just those reasons.
Furthermore this allows relying on all the work of the common-api-runtime. So this would be a perfectly reasonable approach.

The advantage of doing all this in Rust should be that one can run the parsing and code generation at Rust project build time similar to [tonic-build](https://github.com/hyperium/tonic/tree/master/tonic-build).
Another chance is providing a static executable with no dependencies for generating the code. This should lower the barrier to entry for new developers.
All together, let's see how this turns out. If you want something stable and proven you should go with [franca_arc_tools](https://github.com/COVESA/franca_ara_tools).

# Usage

```Rust
fn main() {
    const INPUT: &str = "
package test

interface HelloWorld {
    version { major 1 minor 0 }

    method sayHello {
        in {
            String name
        }
        out {
            String value_a
            Double value_b
            UInt32 value_c
        }
    }
}
";


    let (_remaining, model) = f_model(INPUT).unwrap();
    println!("{:#?}", model);
}
```

Will print:

```Rust
FModel {
    name: "test",
    imports: [],
    interfaces: [
        FInterface {
            name: "HelloWorld",
            comment: None,
            version: Some(
                FVersion {
                    major: 1,
                    minor: 0,
                },
            ),
            attributes: [],
            methods: [
                FMethod {
                    name: "sayHello",
                    comment: None,
                    fire_and_forget: None,
                    in_args: [
                        FArgument {
                            name: "name",
                            comment: None,
                            type: FTypeRef {
                                predefined: Some(
                                    String,
                                ),
                            },
                            array: None,
                        },
                    ],
                    out_args: [
                        FArgument {
                            name: "value_a",
                            comment: None,
                            type: FTypeRef {
                                predefined: Some(
                                    String,
                                ),
                            },
                            array: None,
                        },
                        FArgument {
                            name: "value_b",
                            comment: None,
                            type: FTypeRef {
                                predefined: Some(
                                    Double,
                                ),
                            },
                            array: None,
                        },
                        FArgument {
                            name: "value_c",
                            comment: None,
                            type: FTypeRef {
                                predefined: Some(
                                    UInt32,
                                ),
                            },
                            array: None,
                        },
                    ],
                    error_enum: None,
                    errors: None,
                },
            ],
            broadcasts: [],
        },
    ],
    type_collections: [],
}
```