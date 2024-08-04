![Bella Logo](./graphics/logo/bella_light_mode.png#gh-light-mode-only)
![Bella Logo](./graphics/logo/bella_dark_mode.png#gh-dark-mode-only)

<div align="center">

# A library for rendering Vector Graphics in Bevy with the power of Vello.

Bella is a library for the **Bevy** game engine that allows you to control **Vello** without having to leave Bevy's ECS.

</div>

# It's extremely easy!

<table>
  <tr>
    <td>This code...</td>
    <td>...spawns a line!</td>
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
<td>

![Bella Line](https://github.com/user-attachments/assets/a7e99806-45d9-41e5-aac8-5bbf30e5ffc3)

</td>
</tr>
</table>

*-WIP-*
