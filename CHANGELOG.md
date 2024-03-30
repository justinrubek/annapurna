# Changelog
All notable changes to this project will be documented in this file. See [conventional commits](https://www.conventionalcommits.org/) for commit guidelines.

- - -
## [0.2.1](https://github.com/justinrubek/annapurna/compare/0.2.0..0.2.1) - 2024-03-30
#### Bug Fixes
- **(web)** properly link service worker from pages other than the root index - ([000d8cf](https://github.com/justinrubek/annapurna/commit/000d8cf0574e70084b0664b8d6497f39fad77d02)) - [@justinrubek](https://github.com/justinrubek)
#### Build system
- **(cargo)** update dioxus - ([28a971d](https://github.com/justinrubek/annapurna/commit/28a971db4fafd5e99a2a39c5eaefe11529c73d9a)) - [@justinrubek](https://github.com/justinrubek)
- **(cargo)** cargo update - ([d0db254](https://github.com/justinrubek/annapurna/commit/d0db25404247249c0eef32e74ea560681ad19bb6)) - [@justinrubek](https://github.com/justinrubek)
- **(cargo)** update axum to 0.7 - ([cb0ea66](https://github.com/justinrubek/annapurna/commit/cb0ea66f683c28f41b0ae2b0c87e7ebab7f410d4)) - [@justinrubek](https://github.com/justinrubek)
- **(flake-parts/web)** include wasm modules from nix - ([8e76199](https://github.com/justinrubek/annapurna/commit/8e76199f842cac9ed2326836f516492520761b40)) - [@justinrubek](https://github.com/justinrubek)
- **(nix)** update fenix input - ([ded34d6](https://github.com/justinrubek/annapurna/commit/ded34d6a44c017574fc29c2d6bcf91563b29b3a6)) - [@justinrubek](https://github.com/justinrubek)
- **(nix)** add wasm-bindgen flake input - ([fdf3d49](https://github.com/justinrubek/annapurna/commit/fdf3d49a3818260e7e7c5c931071152d9d8a9a18)) - [@justinrubek](https://github.com/justinrubek)
- **(nix)** move facts package into web file - ([33ccb3c](https://github.com/justinrubek/annapurna/commit/33ccb3cfafcde4919b78b1f2126ef784d30b64ea)) - [@justinrubek](https://github.com/justinrubek)
- **(web)** provide rust service-worker in static-files - ([b9e7871](https://github.com/justinrubek/annapurna/commit/b9e7871b3ba9cbaf53d98defc8ed5c5bceb329fe)) - [@justinrubek](https://github.com/justinrubek)
#### Continuous Integration
- **(github/actions)** add container build action - ([c05ab3d](https://github.com/justinrubek/annapurna/commit/c05ab3dc735d40b6e575d100d902bed3ca142786)) - [@justinrubek](https://github.com/justinrubek)
- **(github/actions)** add github release action - ([b42e21a](https://github.com/justinrubek/annapurna/commit/b42e21aa3d82cb80501317987967d6263410d6f7)) - [@justinrubek](https://github.com/justinrubek)
- **(pre-commit)** add sqlx prepare hook - ([d38b481](https://github.com/justinrubek/annapurna/commit/d38b481d1149de2f19b08b18f9eafd6040687daa)) - [@justinrubek](https://github.com/justinrubek)
#### Features
- **(http)** add sqlx connection to axum server state - ([2409fcc](https://github.com/justinrubek/annapurna/commit/2409fcc8823aa938ae0d9b71ef5ebd4f2d9ceb50)) - [@justinrubek](https://github.com/justinrubek)
- **(http)** add /health route - ([2859867](https://github.com/justinrubek/annapurna/commit/2859867a08d75e3e1badfd1f0d3a82d71db4e354)) - [@justinrubek](https://github.com/justinrubek)
- **(postgres)** add sqlx and migrations - ([508a078](https://github.com/justinrubek/annapurna/commit/508a078c9189fdd192fa209d79f884eef729bc22)) - [@justinrubek](https://github.com/justinrubek)
- **(service-worker)** check if token is expired - ([1f87496](https://github.com/justinrubek/annapurna/commit/1f874966c6a111996b54544fffc070e57a03e0b0)) - [@justinrubek](https://github.com/justinrubek)
- **(service-worker)** implement fetch request interception in rust - ([99376dd](https://github.com/justinrubek/annapurna/commit/99376dd2dd0790d14506f5885819d454dc31add2)) - [@justinrubek](https://github.com/justinrubek)
- **(service-worker)** redirect to home upon login - ([ca9bb66](https://github.com/justinrubek/annapurna/commit/ca9bb663e438eb36c13799e98ab51c1098220434)) - [@justinrubek](https://github.com/justinrubek)
- **(service-worker)** Add key-value store - ([6810a64](https://github.com/justinrubek/annapurna/commit/6810a6486e16d0e5f41e9fd4c7a6cacb639d0a12)) - [@justinrubek](https://github.com/justinrubek)
- **(service-workout)** implement Logout message - ([243cd4c](https://github.com/justinrubek/annapurna/commit/243cd4c2ee23f41291986fd1675a94587e177690)) - [@justinrubek](https://github.com/justinrubek)
- **(services)** added postgres service - ([42dd383](https://github.com/justinrubek/annapurna/commit/42dd3836384083d179658c326a7902c5002d19e8)) - [@justinrubek](https://github.com/justinrubek)
- allow creating and listing `inventory` objects - ([e26bb76](https://github.com/justinrubek/annapurna/commit/e26bb7621837f309d3fa57562a89861c49dce778)) - [@justinrubek](https://github.com/justinrubek)
- add container image package - ([a22ddf7](https://github.com/justinrubek/annapurna/commit/a22ddf781cf0765d2153d35df14e055786eeffa8)) - [@justinrubek](https://github.com/justinrubek)
- initialize service-worker crate - ([15801e9](https://github.com/justinrubek/annapurna/commit/15801e97f607bcd90d0819eacd574469286a554c)) - [@justinrubek](https://github.com/justinrubek)
#### Miscellaneous Chores
- remove javascript service-worker - ([53ba7d7](https://github.com/justinrubek/annapurna/commit/53ba7d7f8e9bcc7d72133f7f1c70589b83c05a3d)) - [@justinrubek](https://github.com/justinrubek)
#### Refactoring
- **(flake-parts/web)** parameterize the copying of wasm-modules - ([80c6979](https://github.com/justinrubek/annapurna/commit/80c6979c8cf7b32f303dfb24abc0dcc3ae02d311)) - [@justinrubek](https://github.com/justinrubek)
- **(http)** dynamically determine login callback-url - ([7d2faeb](https://github.com/justinrubek/annapurna/commit/7d2faebf0ff2051cc392893a15e63ac14c2553d7)) - [@justinrubek](https://github.com/justinrubek)
- **(service-worker)** rework service worker application boundaries - ([59cd96f](https://github.com/justinrubek/annapurna/commit/59cd96f2107a948a611aa410fa977796476c8e65)) - [@justinrubek](https://github.com/justinrubek)
- rename wasm-modules to es-modules - ([94989e0](https://github.com/justinrubek/annapurna/commit/94989e0275ed2dcc4959184749e2ab0b5e5c4be5)) - [@justinrubek](https://github.com/justinrubek)

- - -

## [0.2.0](https://github.com/justinrubek/annapurna/compare/0.1.0..0.2.0) - 2023-09-24
#### Build system
- **(cargo)** cargo update - ([039a8fc](https://github.com/justinrubek/annapurna/commit/039a8fcd59cc2bbb6a11684fe710d5078cdfda4a)) - [@justinrubek](https://github.com/justinrubek)
- **(cargo)** update dioxus - ([10a54ef](https://github.com/justinrubek/annapurna/commit/10a54ef7ab0a3fde28339c2acef9984b437dda89)) - [@justinrubek](https://github.com/justinrubek)
- **(cargo)** cargo update - ([aaac3db](https://github.com/justinrubek/annapurna/commit/aaac3db6d3fd0a4e04957bf313aa0e61957bae97)) - [@justinrubek](https://github.com/justinrubek)
- **(cargo)** exclude wasm from non-wasm builds - ([93c5644](https://github.com/justinrubek/annapurna/commit/93c5644fae29cfbc4aad65c5175496209ffd848a)) - [@justinrubek](https://github.com/justinrubek)
- **(nix)** update flake lock - ([23dbe56](https://github.com/justinrubek/annapurna/commit/23dbe564c7f430ec04b60595d7fb899cbcbecd39)) - [@justinrubek](https://github.com/justinrubek)
- **(nix)** update nixpkgs input - ([8cf979c](https://github.com/justinrubek/annapurna/commit/8cf979c5ad120c96102823306631e00158a4d5d0)) - [@justinrubek](https://github.com/justinrubek)
- **(nix)** update fenix input - ([04e4d07](https://github.com/justinrubek/annapurna/commit/04e4d07404484761e5b0de5eacad3e9617db75ab)) - [@justinrubek](https://github.com/justinrubek)
#### Continuous Integration
- add cocogitto config - ([a3ae7ee](https://github.com/justinrubek/annapurna/commit/a3ae7ee6fa3635301e405e4b94992cfab0f97833)) - [@justinrubek](https://github.com/justinrubek)
#### Features
- **(ui)** recipe logic page - ([f73fce7](https://github.com/justinrubek/annapurna/commit/f73fce7176e4d8c3a8c557a233dcdc3c5acb0a2d)) - [@justinrubek](https://github.com/justinrubek)
- **(ui)** inventory page - ([30d48a9](https://github.com/justinrubek/annapurna/commit/30d48a9cf4ce060fa9e5079101539ced1362251d)) - [@justinrubek](https://github.com/justinrubek)
- **(ui)** create ingredients - ([c12f530](https://github.com/justinrubek/annapurna/commit/c12f530f907c19dd0e478ccc80659a7d4ad79cdc)) - [@justinrubek](https://github.com/justinrubek)
- create inventory items - ([e3be1dd](https://github.com/justinrubek/annapurna/commit/e3be1ddbab115caf84057ee5e4b3d828100e6127)) - [@justinrubek](https://github.com/justinrubek)
- recipe download button - ([f2fd76b](https://github.com/justinrubek/annapurna/commit/f2fd76b9b38ba2e41a65bca0161f2f54223a32d6)) - [@justinrubek](https://github.com/justinrubek)
- navigation component - ([4a03022](https://github.com/justinrubek/annapurna/commit/4a03022ed63a398157a29f37bf69374e23988727)) - [@justinrubek](https://github.com/justinrubek)
- create recipes web ui - ([9fb7431](https://github.com/justinrubek/annapurna/commit/9fb74318d67b385d078b9457a7a33a54b6e3cca5)) - [@justinrubek](https://github.com/justinrubek)
- load recipes from backend - ([38203e9](https://github.com/justinrubek/annapurna/commit/38203e97c53faf734b1fc1857a53c0304db75a40)) - [@justinrubek](https://github.com/justinrubek)
- introduce AppState - ([a2d0d3d](https://github.com/justinrubek/annapurna/commit/a2d0d3da9dd630eed57feafa8575639f36f8eb2f)) - [@justinrubek](https://github.com/justinrubek)
- introduce dioxus-router - ([ba4a63e](https://github.com/justinrubek/annapurna/commit/ba4a63eb43ab1133799f8b6deb028edf5f01b80c)) - [@justinrubek](https://github.com/justinrubek)
- add /api/recipes route - ([7c90d13](https://github.com/justinrubek/annapurna/commit/7c90d1368e6610da2cb74f55e6432f5017f7b3f1)) - [@justinrubek](https://github.com/justinrubek)
- add Facts struct for loading data - ([d596a38](https://github.com/justinrubek/annapurna/commit/d596a381a5283f73cbeff9e15e80e412d862014a)) - [@justinrubek](https://github.com/justinrubek)
- add Recipe component - ([8bf81ee](https://github.com/justinrubek/annapurna/commit/8bf81ee57ce956b1eb3a8280e7897f5a59a35b41)) - [@justinrubek](https://github.com/justinrubek)
- introduce dioxus - ([116475e](https://github.com/justinrubek/annapurna/commit/116475e39d25fbc6bef27933b8da945c12eb8e12)) - [@justinrubek](https://github.com/justinrubek)
- Add serviceworker to handle token - ([d1ce757](https://github.com/justinrubek/annapurna/commit/d1ce757c5e1a0556e04e51da72868c41994c3d75)) - [@justinrubek](https://github.com/justinrubek)
- Implement login redirect - ([b89a9ea](https://github.com/justinrubek/annapurna/commit/b89a9eab1bd6b6af0b127c7c796011228e10bfba)) - [@justinrubek](https://github.com/justinrubek)
- dev command determines which directory based on git repo - ([9f00987](https://github.com/justinrubek/annapurna/commit/9f00987dc96e44393e387b5f6321d26c0bfbfa60)) - [@justinrubek](https://github.com/justinrubek)
- Server development mode - ([d8cb304](https://github.com/justinrubek/annapurna/commit/d8cb304ff47c17dd37053ca7fb1d2a39a00566e9)) - [@justinrubek](https://github.com/justinrubek)
- Implement debounced watcher - ([ca8bb33](https://github.com/justinrubek/annapurna/commit/ca8bb3380e6108497bb384089e0c578ee0f75cf8)) - [@justinrubek](https://github.com/justinrubek)
- Add watch server for frontend - ([7b0c915](https://github.com/justinrubek/annapurna/commit/7b0c9157c52413f307a1dd8fc3ee456064ad4717)) - [@justinrubek](https://github.com/justinrubek)
- Serve site contents - ([b14fefd](https://github.com/justinrubek/annapurna/commit/b14fefdff9d5a90a85e0b36e4b60e8d28c5a0bb5)) - [@justinrubek](https://github.com/justinrubek)
- Add axum web server - ([9aa1260](https://github.com/justinrubek/annapurna/commit/9aa126018b6cec8bdbea337be39ee798f133a19c)) - [@justinrubek](https://github.com/justinrubek)
- introduce astro web site - ([63b0041](https://github.com/justinrubek/annapurna/commit/63b0041d0fe71ef2497a02fab54850ca00cfb1c8)) - [@justinrubek](https://github.com/justinrubek)
#### Miscellaneous Chores
- **(cli)** remove dev command - ([a8d6035](https://github.com/justinrubek/annapurna/commit/a8d6035c749ed900417ceb42fe021d8edf16b139)) - [@justinrubek](https://github.com/justinrubek)
- **(cli)** remove watcher crate - ([d7937e1](https://github.com/justinrubek/annapurna/commit/d7937e1037af83b202bec7784e834b4411f4d8dc)) - [@justinrubek](https://github.com/justinrubek)
- update async-watcher - ([3855cbc](https://github.com/justinrubek/annapurna/commit/3855cbc4ea26e689f32021298b949325a7aba380)) - [@justinrubek](https://github.com/justinrubek)
- introduce new pages - ([6c13ffa](https://github.com/justinrubek/annapurna/commit/6c13ffa53d0615ff91899d2a560e3b5e2bd5c9af)) - [@justinrubek](https://github.com/justinrubek)
- move facts directory to the repository top level - ([db83f9c](https://github.com/justinrubek/annapurna/commit/db83f9c35dfed2097c46f12f293d1ef9aad2a3f3)) - [@justinrubek](https://github.com/justinrubek)
- Remove astro - ([79a35ae](https://github.com/justinrubek/annapurna/commit/79a35ae412b5128f9570f52b3e30944933657cec)) - [@justinrubek](https://github.com/justinrubek)
#### Refactoring
- **(cli)** build static web files using nix - ([f4520dd](https://github.com/justinrubek/annapurna/commit/f4520ddd464a6dbf8aad789bc6df5649635ab7d1)) - [@justinrubek](https://github.com/justinrubek)
- **(cli)** Use reverse proxy for dev mode - ([2ca179d](https://github.com/justinrubek/annapurna/commit/2ca179df3d0dfadf7a47747943cb15ffc4f2c545)) - [@justinrubek](https://github.com/justinrubek)
- **(logic)** `recipe` function now returns data - ([550ac9e](https://github.com/justinrubek/annapurna/commit/550ac9eb0ecee31d2dc401f0f211fcfc844911af)) - [@justinrubek](https://github.com/justinrubek)
- **(nix)** update flake-parts modules - ([ca09698](https://github.com/justinrubek/annapurna/commit/ca09698d28db57ba1bd07df4e0307df9fa535dd0)) - [@justinrubek](https://github.com/justinrubek)
- **(ui)** move datalist definition into toplevel component - ([022cfdd](https://github.com/justinrubek/annapurna/commit/022cfdd8fcaa87cbe6aa0f1cb6d04da514cbd3da)) - [@justinrubek](https://github.com/justinrubek)
- load recipes in toplevel app component - ([3f630b7](https://github.com/justinrubek/annapurna/commit/3f630b7edac95ff64ae7988fdfe5064e4e81f9c3)) - [@justinrubek](https://github.com/justinrubek)
- turn RecipeCreateProps into CreateFormProps - ([949fd22](https://github.com/justinrubek/annapurna/commit/949fd22fe8604d6c8855b5e2e208ff02ef41019a)) - [@justinrubek](https://github.com/justinrubek)
- move types into separate crate - ([af3e87f](https://github.com/justinrubek/annapurna/commit/af3e87f407872ba773f2d0d466fc11ff0e9cc236)) - [@justinrubek](https://github.com/justinrubek)
- bypass astro - ([b60d9ee](https://github.com/justinrubek/annapurna/commit/b60d9ee7f26f22e6dfca80279fbd8b947afd20d7)) - [@justinrubek](https://github.com/justinrubek)
- move http injection logic into separate module - ([60c20c9](https://github.com/justinrubek/annapurna/commit/60c20c956f5f93fde9fb8dbf22e6840a8c5de507)) - [@justinrubek](https://github.com/justinrubek)

- - -

Changelog generated by [cocogitto](https://github.com/cocogitto/cocogitto).