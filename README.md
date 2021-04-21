<div align="center">
	<h1>gig</h1>
	<p>
		Simple cli to create a .gitignore based off <a href="https://github.com/github/gitignore">Github's gitignore</a> repo
	</p>
	<br>
</div>

## Usage

```shell
$ gig -l # lists out all possible .gitignore
$ gig <keyword> # will print to stdout specify .gitignore
$ gig <keyword> --write # Similar to > .gitignore but is careful not to overwrite
$ gig <keyword> --write-force # Force writes to .gitignore
```

### Example
```shell
$ gig -l | grep -i ^c | head -n 3
C++.gitignore
C.gitignore
CFWheels.gitignore
$ gig c++ --write 
Writing c++ gitignore to .gitignore...
$ gig c >> .gitignore
$ gig rust --write-force
```

## Install

### Cargo

If you're using a recent version of Cargo, you can see the `cargo install` command:

```shell
$ cargo install gig-cli
```


### Build from source

After git cloning this repo, you can install as a cargo crate through

```shell
$ cargo install --path path/to/repo
```
## License

MIT

Maintained by [Milan](https://mdaverde.com)
