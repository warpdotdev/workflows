# [0] Overview and Mockups

Entering commands in the terminal can be tedious and hard–developers often need to construct commands one argument and flag at a time, often using StackOverflow.

Workflows (formerly called Tasks) make it easy to browse, search, execute and share commands (or a series of commands)--without needing to leave your terminal.

Warp wants to make it easier to enter commands by launching with 100+ common workflows and an open-source file format--so devs can create private workflows for themselves or their team.

Please comment with feedback, we’ll create an individual sub-thread for each mockup and memo section!



|[A] The Workflow’s menu.|
|:--:|
| ![A](https://user-images.githubusercontent.com/29553206/150206227-e5ae4eec-b322-4509-aba8-f67ee23eab39.png) |

|[B] Filling a Workflow in the Input Editor with the menu expanded.|
|:--:|
| ![B](https://user-images.githubusercontent.com/29553206/150206255-2288326d-4c5c-459a-b1fc-77c046c13bc7.png) |

|[C] Filling a Workflow in the Input Editor with the menu collapsed and tab completion.|
|:--:|
| ![C](https://user-images.githubusercontent.com/29553206/150207821-d0d02b9f-686e-475a-b1fe-a3741fb3f707.png) |

# [1] Pain Points and Goals

Our team, tends to run into a few pain points when entering commands:

1. We need to find a command but
	1. the arguments and flags do not represent the task we are trying to perform
		1. To “undo last git commit” -> `git reset --hard SOFT`
	2. our team's playbook (guide) lives outside of the terminal 
2. We can’t find aliases that other team members have made
	1. No store to browse aliases
	2. Sharing, revoking access, or updating aliases through version control is not straightforward, especially
	   since aliases are global
3. The shell does not provide enough documentation or UI to help enter parameters to commands

# [2] How is this Different from Aliases?

Power users tend to save aliases, create shell functions, and leverage CLI tools that streamline this.
Aliases, however, have major pain points:

1. need to context switch
	1. leave vim, source dotfiles, or reset shell
2. it’s difficult to attach documentation
3. are not easily searchable or sharable
4. are not easily parameterized

Getting aliases and functions to a productive state requires an upfront investment that’s justifiable for devs who
spend most of their workday in the terminal but less so for beginners and casual users.

# [3] Workflows V1

We’re planning on the first launch of Workflows to include:

1. The ability to search and execute a workflow directly from the input editor
2. The top 100 workflows across various commands already bundled
   1. `git`, `sed`, and `grep`
3. The ability for devs to create private workflows in an open-source file-format, including repo-specific workflows that live in a `.warp` directory
4. The ability to create a workflow from a Block by right-clicking on it

# [4] S/O Open Source Projects

These are some of the open source projects that we’ve gotten inspiration from: [tldr;](https://github.com/tldr-pages/tldr) [cheat.sh](https://github.com/chubin/cheat.sh), [cheatsheets](https://github.com/rstacruz/cheatsheets), [navi](https://github.com/denisidoro/navi), [howdoi](https://github.com/gleitz/howdoi), [cheat](https://github.com/cheat/cheat), [how2](https://github.com/santinic/how2), [kb](https://github.com/gnebbia/kb), [eg](https://github.com/srsudar/eg).

Navi [in particular!](https://github.com/denisidoro/navi/blob/master/docs/cheatsheet_syntax.md)

# [5] Question for the Community

We’ve made a thread for each section; please reply in the appropriate threads to help organize discussion!

### We'll ship with some prebuilt workflows, what are some workflows to include? 
If you can, try to use this format:

#### Title of workflow (what does it do)

Extract a field from a json

#### Command

`cat channel_versions.json | jq '.stable.versions'`

#### Parameters

`channel_versions.json` -> a json file

#### Articulation or links to StackOverflow (or other sites)

https://stackoverflow.com/questions/39228500/extract-a-specific-field-from-json-output-using-jq
