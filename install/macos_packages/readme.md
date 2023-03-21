# MacOS Codesigning Scripts

Well, here we are. The Apple notarization procedure is complex enough that I
need an actual pile of scripts and writeup to be able to remember how to do it.

The basic procedure is as follows:

- Build code
- Build docs
- Sign binary with Developer Application ID
- Upload binary to notarization service to get notarized
- Use code + docs to generate a component package
- Use component package to generate a distribution package
- Sign distribution package with Developer Installer ID
- Upload distribution package to notarization service to get notarized

Et. voila, you have a notarized distribution package which can be installed.

I fully anticipate that this procedure will break in the future, so here is the
(scant) documentation that I have been able to scrape together on these procedures,
along with some commentary on things that I've found and requirements for these
scripts.

You will need XCode installed.

## Short-form Command Line Invocation

If you have the prerequisites set up (including the environment variables as
described below, built docs, and built release binary), you can generate the
package file with the following command:

```
./install/macos_packages/build_and_notarize.sh target/release/starship docs x64
```

or `arm64` if building on Apple silicon.

## Setting Up Credentials

### Apple Developer Account

In order to get the signing keys, you need to have a developer account. You can
buy one at https://developer.apple.com/programs/ for $100 a year (at time of
writing).

There is no other way to acquire an account, which is needed to obtain
non-self-signed keys and to be able to notarize files.

### Signing Keys

To generate the signing keys, I went through the [XcodeGUI](https://help.apple.com/xcode/mac/current/#/dev154b28f09), though there are
several other methods to do this. You will need at least one Application signing
key and one Installer signing key.

To check what signing keys are available, you can use the following command:

```
security find-identity -p basic -v
```

### Notarization Credentials

To be able to notarize objects, you will need an app-specific password. You will
need to set it up using the instructions [on this page](https://support.apple.com/en-us/HT204397).
You will also need your team ID, which can be found at https://developer.apple.com/account/#/membership
(if it goes to the home page, click on "Membership" on the left panel), and your
Apple ID (usually an email address).

If you want to enter everything manually, most commands that require these values
accept the `--apple-id`, `--team-id`, and `--password` flags. However, I find it
simpler to store the credentials in the keychain. You can do so with the following
command:

```
xcrun notarytool store-credentials "<AUTH_ITEM_NAME>" --apple-id "<apple-id>" --password "<password>" --team-id "<team-id>"
```

where `<AUTH_ITEM_NAME>` is a name you will use later to refer to the credentials,
and the other three items are the Apple ID, the Team ID, and the app-specific password,
respectively. For the rest of this document, I will assume that its value is
`AC_PASSWORD` for compatibility with Apple's website, though you may choose
whatever you like.

### Script Assumptions

The scripts in this directory assume that the signing keys and the notarization
credentials are unlocked and available within a specific keychain file, stored
in a file at `$RUNNER_TEMP/$KEYCHAIN_FILENAME`. Additionally, it assumes that
the `AUTH_ITEM_NAME` used to refer to the notarization credentials is found in
the environment under the variable `KEYCHAIN_ENTRY`.

The CI environment ensures that the keychain file exists at the appropriate
locations and is destroyed after use. If you are running these scripts locally,
the values that correspond to what Apple uses in their tutorials are:

```
KEYCHAIN_ENTRY=AC_PASSWORD   # Or whatever you picked for <AUTH_ITEM_NAME> above
RUNNER_TEMP=~/Library/Keychains
KEYCHAIN_FILENAME=login.keychain
```

Note to developers: because the keychain file may be a user's personal keychain,
you MUST NEVER WRITE TO THE KEYCHAIN FILE in these scripts. On the CI, CI actions
will ensure that the keychain file is shredded after use.

## Codesigning a Binary

This is actually fairly simple. Run

```
codesign --timestamp --sign "<Key ID>" --verbose -f -o runtime <binary>
```

to sign the binary file. `--timestamp` is not required for signing, but will be
required for notarization. `<Key ID>` can be one of two things: the name of the
signing key (on the right of `security find-identity -p basic -v`), or the key
hash (the hex string on the left of the command).

Usually you can use name of the key, but if you have multiple keys like me, you
may need to use the hex string to specify.

## Notarizing a Binary

Once the binary has been signed, you need to package it into a .zip file in order
to be able to send it to Apple for notarization. The simplest way to do this is
to run `zip <archive.zip> <binary>`.

Then, run `xcrun notarytool submit <archive.zip> --keychain-profile "AC_PASSWORD" --wait`
to submit the binary for notarization. The `--wait` flag will cause the tool to
block until the notarialization is complete. If you want to be able to leave and
check the results later, omit `--wait` (though starship notarization usually takes
no more than 60s).

Finally, you should check the submission logs. To get a record of all notarization
attempts, run

```
xcrun notarytool history --keychain-profile "AC_PASSWORD"
```

Find the `id` of the attempt you wish to view, then run one of these commands:

```
xcrun notarytool info <run-id> --keychain-profile "AC_PASSWORD"
xcrun notarytool log <run-id> --keychain-profile "AC_PASSWORD"
```

The `log` command downloads a JSON log of the notarization attempt, and can reveal
warnings that should be fixed before the next submission.

Additional details on the notarization process can be found at https://developer.apple.com/documentation/security/notarizing_macos_software_before_distribution/customizing_the_notarization_workflow.
Note that while Apple has a lot of requirements on their pages, including stuff
like Hardened Runtime requirements and listing entitlements, as far as I can tell,
starship does not require any of these even though we do things like send
notifications and access the network via an HTTP client. Nonetheless, I'm
linking the [entitlements page](https://developer.apple.com/documentation/bundleresources/entitlements)
here in case it becomes important later.

## Creating a Component Package

Since I'm only dealing with one binary, we will make one Component package and
one Distribution package. Surprisingly, the flat package (.pkg) format is not
documented by Apple. [This guide](https://matthew-brett.github.io/docosx/flat_packages.html)
and many of the links within it are the best documentation available on the subject.

To build a component package, we first need to create a temporary directory and
create a pseudo-filesystem within it (similar to makepkg on Arch). For example,
if we place the directory at `$TEMP_DIR/usr/local/bin/starship`, the binary
will be installed at `/usr/local/bin/starship` once the installer runs.

An aside on docs: We would also like to include documentation in the pkg.
Unfortunately, Vuepress currently cannot build with relative paths, and any
attempt at hacking this in seems to create even more problems. Instead, the
scripts do the dumbest thing imaginable: build the documentation, serve it with
a simple HTTP server, and then use `wget` to make a local copy which can be
viewed offline.

Once everything is placed in the correct locations, we can run the following
command to generate the component package:

```
pkgbuild --identifier com.starshipprompt.starship --version "<version>" --root <pkgdir> output.pkg
```

## Notarizing the Component Package (and why we don't need to)

Fortunately for us, Apple has confirmed that we only need to notarize the
[outermost installer mechanism](https://developer.apple.com/forums/thread/122045).

Therefore, if we are sending the component package on its own, we should notarize
it now. However, for starship, we will bundle this into a distribution package,
so we don't need to notarize this pkg file.

## Creating a Distribution Package

To create a distribution, we do the following steps:

- Use `productbuild` to generate a skeleton distribution file.
- Insert custom welcome/license/conclusion and icon files into the installer.
- Build the installer with `productbuild`.

I have elected not to make a fat binary due to concerns over startup cost, so
there are two .plist files that can be used to specify the architecture required.

## Signing the Distribution package

This is also fairly simple, and analagous to signing the binary.

```
productsign --timestamp --sign "<Key ID>" <input.pkg> <output.pkg>
```

## Notarizing the Distribution Package

Also analagous to notarizing the binary. We run

```
xcrun notarytool submit <package.pkg> --keychain-profile "AC_PASSWORD" --wait
```

and also check the submission logs.

Note: you may need to enter your password a ridiculous number of times (like 4+)
in order to successfully notarize this.

## Stapling the Result

Finally, we staple the notarization ticket to the package, ensuring that anyone
who downloads the file can see that the installer was notarized:

```
xcrun stapler staple <package>
```

Note that `.dmg`, `.app`, and `.pkg` files can be stapled, but `.zip` and
binary files cannot. Distributing the latter files alone will require that the
installing computer can access the internet to verify notarization of the app.

## Putting It All Together

If you don't want to run these commands, a full workflow is available in
`build_and_notarize` script. Check the documentation at the top of the script
for environment variables and arguments that need to be set--it is a fairly
complicated script, but this is a fairly complicated procedure.

# Testing Notarization

To test if a particular item is notarized, run one of the following commands:

```
codesign --test-requirement="=notarized" --verify --verbose <file>
spctl -a -vvv -t install <file>
```

# External Links

https://developer.apple.com/documentation/security/notarizing_macos_software_before_distribution

https://developer.apple.com/documentation/security/notarizing_macos_software_before_distribution/customizing_the_notarization_workflow

https://github.com/akeru-inc/xcnotary

https://www.reddit.com/r/rust/comments/q8r90b/notarization_of_rust_binary_for_distribution_on/
