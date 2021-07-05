
## Running the project

Once the project is built, run the command below. Replace Replace `<application_id>` and `<project_name>` with the values you entered during project creation. Please note that these commands are just for demonstration purposes. Normally this would be handled by your IDE, such as GNOME Builder or VS Code with the Flatpak extension.

```
flatpak-builder --run flatpak_app build-aux/<application_id>.Devel.json <project_name>
```

## Credits

  - Based on this [`gtk-rust-template`](https://gitlab.gnome.org/bilelmoussaoui/gtk-rust-template)
  
