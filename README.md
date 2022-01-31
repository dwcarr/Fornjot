# Fornjot

[**Blog**](https://www.fornjot.app/blog/) | [**Matrix**](https://matrix.to/#/#fornjot:braun-odw.eu) | [**Discussions**](https://github.com/hannobraun/Fornjot/discussions) | [**Contribution Guide**](CONTRIBUTING.md)

## About

Fornjot is a project to create a **next-generation Code-CAD application**. Because [**the world needs another CAD program**](https://github.com/sponsors/hannobraun).

![Screenshot of the star model](https://github.com/hannobraun/fornjot/blob/main/models/star/star.png)

Fornjot is working towards a long-term vision, following these principles:

- **Write code to create flexible models:** CAD models in Fornjot are defined as code. Rust is supported as the modeling language, but Fornjot is architected to be language-agnostic.
- **Avoid repetitive work:** With the power of a programming language at your disposal, there's no reason to repeat yourself. Creating intricate models doesn't have to be a monotonous chore.
- **The right tool for your job:** Many other Code-CAD applications come with a limited feature set. At this early stage, Fornjot is even worse, but it is built from the ground up to be able to support even advanced CAD features.
- **Getting out of your way:** Fornjot's job is to assist you, by being available where you need it (with planned support for all major desktop and mobile platforms, and the web), being convenient to use, and having the performance to support your workflow.
- **Free and open source**: Fornjot is open source software, available under a permissive license. Try it out for free, with no strings attached. Modify it to your needs. If it's useful to you, decide for yourself how you want to support its further development.

It's still early days, and it will take a long time before Fornjot can truly achieve this vision. Please consider supporting the project, by [helping out yourself](CONTRIBUTING.md) or [contributing financially](https://github.com/sponsors/hannobraun).

This is Fornjot's main repository. The [website is maintained in a separate repository](https://github.com/hannobraun/www.fornjot.app).


## Sponsors

Fornjot is supported by [**@lthiery**](https://github.com/lthiery), [**@Yatekii**](https://github.com/Yatekii), [**@ahdinosaur**](https://github.com/ahdinosaur), [**@jessebraham**](https://github.com/jessebraham), and [**@Kethku**](https://github.com/Kethku).

**Please consider [supporting me too](https://github.com/sponsors/hannobraun), to make Fornjot sustainable long-term.**


## Status

Fornjot is **under active development, but still experimental**. Efforts are currently focused on incrementally expanding features, to support more kinds of CAD models, and plugging holes in the algorithms used, to solidify the existing functionality. Right now, there still many such holes, as many of the algorithms make simplifying assumptions that might not be true for your model.

Priorities have recently shifted towards addressing that, but it will take a while to fix everything. If you are interested in Fornjot and are considering to use it, you should fully expect to run into limitation pretty much immediately. Unless you are willing to contribute to its development, it would be better to wait for a year or ten, to let it mature.

At this point, performance is not much of a concern. This will change, but for now, ease of development is much more important than scaling to more complex models.


## Features

### Code-CAD in Rust

Models are defined as Rust code. To ensure fast compile times, they are compiled separately, and loaded into a host application as a plug-in.

``` rust
use std::collections::HashMap;

#[no_mangle]
pub extern "C" fn model(args: &HashMap<String, String>) -> fj::Shape {
    let outer = args
        .get("outer")
        .unwrap_or(&"1.0".to_owned())
        .parse()
        .unwrap();
    let inner = args
        .get("inner")
        .unwrap_or(&"0.5".to_owned())
        .parse()
        .unwrap();
    let height = args
        .get("height")
        .unwrap_or(&"1.0".to_owned())
        .parse()
        .unwrap();

    let outer_edge = fj::Circle { radius: outer };
    let inner_edge = fj::Circle { radius: inner };

    let footprint = fj::Difference {
        a: outer_edge.into(),
        b: inner_edge.into(),
    };

    let spacer = fj::Sweep {
        shape: footprint.into(),
        length: height,
    };

    spacer.into()
}
```

This is the code for the [spacer model](/models/spacer). As you can see, there's still some work to do, to make the process of defining models more convenient.

### Basic modeling features

At this point, Fornjot supports basic 2D shapes (sketches made from lines segments, circles, limited combinations between them), sweeping those 2D shapes along a straight path to create a 3D shape, and some very incomplete support for constructive solid geometry (CSG).

The short- to mid-term priority is to provide solid CSG support, more flexible sketches, and more flexible sweeps (along a circle or helix). Long-term, the plan is to keep adding more advanced CAD modeling features, to support even complex models and workflows.

### Supports the major desktop platforms

As of this writing, Fornjot runs on Linux, Windows, and macOS. The project is primarily developed on Linux, so the other platforms might be subject to bugs. If you want to help out, regularly testing on Windows and macOS, and reporting bugs, is a good way to do so.

Short- to mid-term, the plan is to add support for the web platform, so Fornjot can run in browsers. Long-term, the plan is to additionally support the major mobile platforms.

### Export to 3MF

Exporting models to the [3D Manufacturing Format](https://en.wikipedia.org/wiki/3D_Manufacturing_Format) (3MF), which is used in 3D printing, is supported.


## Usage

### Defining models

Models depend on the [`fj`](/fj) library, which they use to define the geometry. Furthermore, they need to be built as a dynamic library. Just use the examples in the [`models/`](/models) directory as a template.

### Viewing models

To compile and view a model, run it from the host application.

``` sh
# Compile/view the spacer model
cargo run -- -m spacer
```

This invocation expects that the model exists in the `models/spacer` directory, with a package name of `spacer`.

Rotate the model by pressing the left mouse button while moving the mouse. Move the model by pressing the right mouse button while moving the mouse. Zoom with the mouse wheel.

Toggle model rendering by pressing `1`. Toggle mesh rendering by pressing `2`.

So far, the host application is not published on [crates.io](https://crates.io/), and the whole process is not really optimized for being used outside of this repository. Contributions to improve that situations are very welcome.

### Exporting models

To export a model to a 3MF file, run:

``` sh
cargo run -- -m spacer --export spacer.3mf
```

### Model parameters

Some models have parameters that can be overridden. For example, to override the inner and outer radii of the spacer model:

``` sh
cargo run -- -m spacer --arguments outer=8.0 inner=5.0
```


## Community

If you are interested in Fornjot, please consider joining the community. We'd love to have you!


### Questions, Feedback, Discussions

The following venues are best-suited for questions, feedback, or general discussions:

- [Matrix channel](https://matrix.to/#/#fornjot:braun-odw.eu)
- [GitHub Discussions](https://github.com/hannobraun/Fornjot/discussions)


### Bugs, Feature Requests

If you found a bug or have a specific feature request, please use issues on GitHub:

- [List of open issues](https://github.com/hannobraun/Fornjot/issues)
- [Open a new issue](https://github.com/hannobraun/Fornjot/issues/new)

Feel free to check existing issues and add your voice there, if you find one that fits. But if you are unsure or don't have the time for that, don't let that stop you. We'd rather have duplicate issues than not hear about a bug at all.


### Development

To join the Fornjot project as a developer, please fork one of the GitHub repositories and submit a pull request:

- [Main Fornjot repository](https://github.com/hannobraun/Fornjot)
- [Website repository](https://github.com/hannobraun/www.fornjot.app)

If you need some guidance, check out the [contribution guide](https://github.com/hannobraun/Fornjot/blob/main/CONTRIBUTING.md).


## License

This project is open source, licensed under the terms of the [Zero Clause BSD License] (0BSD, for short). This basically means you can do anything with it, without any restrictions, but you can't hold the authors liable for problems.

See [LICENSE.md] for full details.

[`fj`]: https://crates.io/crates/fj
[Zero Clause BSD License]: https://opensource.org/licenses/0BSD
[LICENSE.md]: https://github.com/hannobraun/fornjot/blob/main/LICENSE.md
