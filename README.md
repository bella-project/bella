![Bella Logo](./graphics/logo/bella_light_mode.png#gh-light-mode-only)
![Bella Logo](./graphics/logo/bella_dark_mode.png#gh-dark-mode-only)

<div align="center">

# A library for rendering Vector Graphics in Bevy with the power of Vello.

Bella is a library for the **Bevy** game engine that allows you to control **Vello** without having to leave Bevy's ECS.

</div>

# It's extremely easy!

<table align="center">
  <tr>
    <td>This code spawns a line!</td>
  </tr>
<tr>
<td>

```rs
// First, create a Bella Instance.
let bella_instance = BellaInstance::new(&mut commands);

// Then, create a line.
commands.spawn(bella_instance.shape(bella_line(), Transform::default()));
```
    
</td>
</tr>
<tr>
<td>

![Bella Line](https://github.com/user-attachments/assets/123c6eed-8626-4207-bcbf-f85be4d71ac0)

</td>
</tr>
</table>

*-WIP-*
