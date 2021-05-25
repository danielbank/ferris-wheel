# 🎡 Ferris Wheel

An attempt at building the [Wheel of Names](https://wheelofnames.com/) in Rust.

This project was presented at the [Desert Rust](https://rust.azdevs.org/) meetup.

# Troubleshooting

-   Getting shapes to move

Naively, we may want to try to do something with the Shapes (e.g. `wheel = shapes::Circle {}`) or the ShapeBundles (e.g. `GeometryBuilder::build_as(...)`) to get them to move. However, [Bevy wants us to interact with Components](https://bevy-cheatbook.github.io/programming/queries.html) which are Structs attached to Entities. We attach these components to entities by calling `.insert()` on the ShapeBundle we get back from `GeometryBuilder`. The components could even be an empty structs, we simply need something to get a handle on when we query the system for things to transform.

-   Getting shapes to rotate

Rather than manipulating `transform.translation`, we call `transform.rotate()` and provide a `Quat` (short for [Quaternion](https://en.wikipedia.org/wiki/Quaternion)) to specify the angle we want to rotate by.

If we provide a fixed value, this will be a constant rotation. We can simulate torque by changing the value with each tick. In this example, we do this by subtracting some multiple of `time.seconds_since_startup()`. We have to clamp the resulting value above 0 because if it goes negative, we just start rotating the opposite direction and get oscillatory motion.

-   Getting past an initial wgpu Validation Error

When I tried running the [bevy_prototype_lyon](https://github.com/Nilirad/bevy_prototype_lyon) example code, I was getting the following error:

```
wgpu error: Validation Error

Caused by:
    In a RenderPass
      note: encoder = `<CommandBuffer-(1, 2, Metal)>`
    In a pass parameter
      note: command buffer = `<CommandBuffer-(1, 2, Metal)>`
    attachment's sample count 8 is invalid
```

I fixed this issue by changing the `Msaa { samples: 8 }` to `Msaa { samples: 4 }`. Some light reading on [Multisample anti-aliasing](https://en.wikipedia.org/wiki/Multisample_anti-aliasing#Sample_patterns) revealed that most modern GPUs support 2×, 4×, and 8× MSAA samples. It looks like my 2015 MacBook Pro is starting to show its age.

Relevant example code below:

```
    App::build()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_startup_system(setup.system())
        .run();
```
