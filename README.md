![Bella Logo](./graphics/logo/bella_light_mode.png#gh-light-mode-only)
![Bella Logo](./graphics/logo/bella_dark_mode.png#gh-dark-mode-only)

<div align="center">

# A library for rendering Vector Graphics in Bevy with the power of Vello.

Bella is a library for the **Bevy** game engine that allows you to control **Vello** without having to leave Bevy's ECS.

</div>

# Easy to use!

<table align="center">
  <tr>
    <td>This code spawns a tiny line!</td>
  </tr>
<tr>
<td>

```rs
// First, create a Bella Instance.
let bella_instance = BellaInstance::new(&mut commands);

// Then, create a line inside of the instance.
commands.spawn(bella_instance.shape(bella_line(), Transform::default()));
```
    
</td>
</tr>
<tr>
<td>

![Bella Tiny Line](https://github.com/user-attachments/assets/268aafba-f6a8-4cb8-8cfd-fe833beecbec)

</td>
</tr>
</table>

<table align="center">
  <tr>
    <td>This line is too tiny... Let's make it bigger!</td>
  </tr>
<tr>
<td>

```rs

// Every line has a beginning point and an end point.
commands.spawn(bella_instance.shape(
    bella_line()
        .with_begin(Vec2::new(-100.0, -100.0))
        .with_end(Vec2::new(100.0, 100.0)),
    Transform::default()
));
```
    
</td>
</tr>
<tr>
<td>

![Bella Line](https://github.com/user-attachments/assets/0d7316aa-abe9-4112-ae1d-7c24ac45e4a6)

</td>
</tr>
</table>

<table align="center">
  <tr>
    <td>And make the line's stroke bigger!</td>
  </tr>
<tr>
<td>

```rs
commands.spawn(bella_instance.shape(
    bella_line()
        .with_stroke(BellaStroke::new(10.0))
        .with_begin(Vec2::new(-100.0, -100.0))
        .with_end(Vec2::new(100.0, 100.0)),
    Transform::default()
));
```
    
</td>
</tr>
<tr>
<td>

![Bella Line but Bigger](https://github.com/user-attachments/assets/1eab19fe-ebf9-4768-867e-d20504ff1f3f)

</td>
</tr>
</table>

**Tada!** A custom line in Bella!

# How to add Bella to your Bevy project.

- Since this is not a Cargo crate (yet), first, you'll have to clone this repository: `git clone https://github.com/bella-project/bella`.

> [!WARNING]  
> Make sure to clone the repository outside of your Bevy project.
  
- One that's done, add Bella in your Cargo.toml, by setting the path where you cloned the repo.

```toml
# ...

[dependencies]
bevy = "0.14.1"
bella = { path = "path/to/bella" }

# ...
```

- Done! You added Bella! Now you can compile and run your project!

*-WIP-*
