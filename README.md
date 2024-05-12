<!-- Improved compatibility of back to top link: See: https://github.com/othneildrew/Best-README-Template/pull/73 -->
<a name="readme-top"></a>
<!--
*** Thanks for checking out the Best-README-Template. If you have a suggestion
*** that would make this better, please fork the repo and create a pull request
*** or simply open an issue with the tag "enhancement".
*** Don't forget to give the project a star!
*** Thanks again! Now go create something AMAZING! :D
-->



<!-- PROJECT SHIELDS -->
<!--
*** I'm using markdown "reference style" links for readability.
*** Reference links are enclosed in brackets [ ] instead of parentheses ( ).
*** See the bottom of this document for the declaration of the reference variables
*** for contributors-url, forks-url, etc. This is an optional, concise syntax you may use.
*** https://www.markdownguide.org/basic-syntax/#reference-style-links
-->
[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![MIT License][license-shield]][license-url]


<div align="center">

<h3 align="center">Display settings library</h3>

  <p align="center">
    Small library to manage display settings on Windows
    <br>
    <a href="https://github.com/PearuUu/DisplaySettings/issues/new">Report Bug</a>
    ·
    <a href="https://github.com/PearuUu/DisplaySettings/issues/new">Request Feature</a>
  </p>
</div>



<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
      <ul>
        <li><a href="#built-with">Built With</a></li>
      </ul>
    </li>
    <li>
      <a href="#getting-started">Getting Started</a>
      <ul>
        <li><a href="#prerequisites">Prerequisites</a></li>
        <li><a href="#installation">Installation</a></li>
      </ul>
    </li>
    <li><a href="#usage">Usage</a></li>
    <li><a href="#roadmap">Roadmap</a></li>
    <li><a href="#contributing">Contributing</a></li>
    <li><a href="#license">License</a></li>
    <li><a href="#contact">Contact</a></li>
    <li><a href="#acknowledgments">Acknowledgments</a></li>
  </ol>
</details>



### Built With


[![Rust][rust-lang.org]][Rust-url]


<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- GETTING STARTED -->
## Getting Started

### Installation

* cargo
  ```sh
  cargo add DisplaySettings
  ```
* Cargo.toml
    ```shell
    DisplaySettings = "0.1.0"
    ```

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- USAGE EXAMPLES -->
## Usage

```rust
use DisplaySettings;
```
```rust
use DisplaySettings::get_display_settings;

fn main() {
    let display_settings = get_display_settings();
    for setting in display_settings {
        println!("Width: {}, Height: {}, Refresh Rate: {}", setting.width, setting.height, setting.refresh_rate);
    }
}
```

```rust
use DisplaySettings::{set_display_settings, DisplaySettingsType};

fn main() {
    let settings = DisplaySettingsType {
        width: 1920,
        height: 1080,
        refresh_rate: 60,
    };
    
    let result = set_display_settings(settings);
    if result == 0 {
        println!("Display settings applied successfully!");
    } else {
        println!("Failed to apply display settings. Error code: {}", result);
    }
}

```
_For more examples, please refer to the [Documentation](https://docs.rs/crate/DisplaySettings/latest)_

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- CONTRIBUTING -->
## Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement".
Don't forget to give the project a star! Thanks again!

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- CONTACT -->
## Contact

Discord - \_\_blur\_\_


Project Link: [https://github.com/PearuUu/DisplaySettings](https://github.com/PearuUu/DisplaySettings)

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->
[contributors-shield]: https://img.shields.io/github/contributors/PearuUu/DisplaySettings.svg?style=for-the-badge
[contributors-url]: https://github.com/PearuUu/DisplaySettings/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/PearuUu/DisplaySettings.svg?style=for-the-badge
[forks-url]: https://github.com/PearuUu/DisplaySettings/network/members
[stars-shield]: https://img.shields.io/github/stars/PearuUu/DisplaySettings.svg?style=for-the-badge
[stars-url]: https://github.com/PearuUu/DisplaySettings/stargazers
[issues-shield]: https://img.shields.io/github/issues/PearuUu/DisplaySettings.svg?style=for-the-badge
[issues-url]: https://github.com/PearuUu/DisplaySettings/issues
[license-shield]: https://img.shields.io/github/license/PearuUu/DisplaySettings.svg?style=for-the-badge
[license-url]: https://github.com/PearuUu/DisplaySettings/blob/master/LICENSE
[linkedin-shield]: https://img.shields.io/badge/-LinkedIn-black.svg?style=for-the-badge&logo=linkedin&colorB=555
[linkedin-url]: https://linkedin.com/in/linkedin_username
[product-screenshot]: images/screenshot.png
[rust-lang.org]: https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white
[Rust-url]: https://www.rust-lang.org/
[Next.js]: https://img.shields.io/badge/next.js-000000?style=for-the-badge&logo=nextdotjs&logoColor=white
[Next-url]: https://nextjs.org/
[React.js]: https://img.shields.io/badge/React-20232A?style=for-the-badge&logo=react&logoColor=61DAFB
[React-url]: https://reactjs.org/
[Vue.js]: https://img.shields.io/badge/Vue.js-35495E?style=for-the-badge&logo=vuedotjs&logoColor=4FC08D
[Vue-url]: https://vuejs.org/
[Angular.io]: https://img.shields.io/badge/Angular-DD0031?style=for-the-badge&logo=angular&logoColor=white
[Angular-url]: https://angular.io/
[Svelte.dev]: https://img.shields.io/badge/Svelte-4A4A55?style=for-the-badge&logo=svelte&logoColor=FF3E00
[Svelte-url]: https://svelte.dev/
[Laravel.com]: https://img.shields.io/badge/Laravel-FF2D20?style=for-the-badge&logo=laravel&logoColor=white
[Laravel-url]: https://laravel.com
[Bootstrap.com]: https://img.shields.io/badge/Bootstrap-563D7C?style=for-the-badge&logo=bootstrap&logoColor=white
[Bootstrap-url]: https://getbootstrap.com
[JQuery.com]: https://img.shields.io/badge/jQuery-0769AD?style=for-the-badge&logo=jquery&logoColor=white
[JQuery-url]: https://jquery.com 