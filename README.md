# Ferris Wheel

An attempt at build the [Wheel of Names](https://wheelofnames.com/) in Rust.

This project was presented at the [Desert Rust](https://rust.azdevs.org/) meetup.

# Troubleshooting

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
