# csv-sanity

Preserve your sanity is a world full of malformed, poorly validated CSV files.
Sanitize and transform large CSVs with millions of records quickly and
efficiently.

**NOTE:** csv-sanity is in an alpha state and is subject to breaking changes.
The ruleset file syntax in particular is likely to change in the near future.
I've personally used csv-sanity on a number of projects and it has been
incredibly helpful, but as with most alpha software csv-sanity is provided
as-is and provides no warranty or guarantee. Use at your own risk and double
check your transformed files!

## Purpose

The CSV format is not well-standardized and has many shortfalls when it comes to
storing large numbers of records with complex data formats, but CSVs are
ubiquitous in many realms as a neutral interchange format that most CRMs and
database software can parse and understand.

But what happens when your CRM can only parse ISO 8601 formatted dates and the
CSV you inherited has dates in another format such as the following:

```csv
id,name,signup_date
2,John Doe,11/22/2017
3,Jane Doe,11/28/2017
```

Or you received a CSV of people who you need to contact via a personalized
email, but your contacts' names in the CSV are in ALL CAPS:

```csv
id,first_name,last_name
2,JOHN,DOE
3,JANE,DOE
```

Or you have a CSV that has valid values for the vast majority of records, but 1
out of every 20k records has nonsense values that cause your entire import to
abort:

```csv
id,fist_name,last_name,party_registration
2,Jane,Doe,REP
3,John,Doe,DEM
345,Josh,Smith,HAHAHAHA
```

Or even a CSV that has a few malformed records due to unescaped commas:

```csv
id,first_name,last_name,email
2,Jane,Doe,jane@example.com
3,John,Doe,"i,don't,follow,the,rules"@example.com
```

These are all real problems I've encountered with CSVs over the years. If the
CSV is small enough they can be corrected by hand, but for CSVs with 10k, 100k
or even millions of records correcting by hand simply isn't a viable option.

`csv-sanity` aims to solve the issue of sanitizing large, poorly-validated CSVs.

## Usage

`csv-sanity` is an executable that takes an input CSV to process and a JSON
ruleset file defining the transformation rules to apply:

```bash
csv-sanity [-r RULESET_FILE] <INPUT_FILE>
```

If a path to a ruleset file is not provided via the `-r` option, `csv-sanity`
will look for a file named "ruleset.json" in the current directory.

By default, `csv-sanity` outputs two files to the current directory:
output.csv, which contains the processed CSV with validated and transformed
records, and errors.csv, which contains a list of records and fields that
couldn't be processed and reasons they were rejected. The paths where the output
and error files are output can be overridden via the `-o FILE_PATH` and
`-e FILE_PATH` options, respectively.

## ruleset.json Syntax

Ruleset files are JSON files that define a collection of transformation rules
and the fields to which they should be applied.

The following is an example ruleset JSON file:

```json
{
    "rules": [
        {
            "applicability": {
                "Global": [],
            },
            "transformer": {
                "None": {
                    "regex": "\\A(?:[:cntrl:]|\\s)*\\z"
                }
            },
            "priority": -10
        },
        {
            "applicability": {
                "Global": [],
            },
            "transformer": {
                "Trim": {}
            },
            "priority": -10
        },
        {
            "applicability": {
                "Fields": {
                    "field_names": [
                        "first_name",
                        "last_name"
                    ]
                }
            },
            "transformer": {
                "Capitalize": {}
            }
        }
    ]
}
```

Every ruleset.json file is a JSON object with a single "rules" field with an
array of rule objects.

Rules are objects with two fields:

- **"applicability"**: specifies whether a rule applies globally or only to a
    predefined set of fields (specified as the column headers in the CSV being
    processed)
- **"transformer"**: a transformer object, which specifies how the applicable
    fields should be transformed.

### Transformers



#### Capitalize

Transforms string fields into Capital Case.

#### Choice

Only accepts a pre-defined list of acceptable values and rejects the rest.

#### Date

```json
{
    "Date": {
        "input_formats": [
            "%m/%d/%Y"
        ],
        "output_formats": "%F"
    }
}
```

Attempt to parse fields with a list of datetime formats via
[time::strptime](https://docs.rs/time/0.1.37/time/fn.strptime.html). See the
docs for the [time](https://docs.rs/time/0.1.37/time/index.html) crate for
details on datetime formating syntax.

#### Email

```json
{
    "Email": {}
}
```

Attempt to parse fields as email addresses, rejecting any fields that appear to
be invalid email addresses.

#### None

```json
{
    "None": {
        "regex": "\\A(?:[:cntrl:]|\\s)*\\z"
    }
}
```

Replace matched fields with a blank value. Useful as a global rule for
normalizing blank fields in a CSV file.

#### Number

```json
{
    "Number": {}
}
```

Attempt to parse fields as whole integers, rejecting any fields that cannot be
parsed.

#### PhoneNumber

```json
{
    "PhoneNumber": {}
}
```

Attempt to parse files as US, NANP-formatted phone numbers, transforming them
into a standard international format of `+1 <area_code> <exchange_code> <subscriber_number>`.

#### Regex

```json
{
    "Regex": {
        "regex": "\\A([A-Z])[A-Z]+\\z",
        "template": "$1"
    }
}
```

Match fields against the provided regex pattern and transform them according to
the template string, replacing capture groups placeholders. See the
[Regex::replace](https://docs.rs/regex/0.2.1/regex/struct.Regex.html#method.replace)
in the regex crate docs for details.

#### RegexMatch

```json
{
    "RegexMatch": {
        "regex": "\\A[A-Z]{2,3}\\z",
        "negate": false
    }
}
```

Reject any fields that fail to match against the provided regex pattern. If
`negate` is `true`, the reject any fields that match the provided regex pattern
instead.

#### Trim

```json
{
    "Trim": {}
}
```

Trim leading and trailing whitespace from fields. Useful as a global rule to
normalize fields and remove useless whitespace.

#### Zipcode

```json
{
    "Zipcode": {}
}
```

Attempt to parse fields as US zip codes in the formats "xxxxx" and "xxxxx-xxxx",
rejecting any fields that fail to match that format.
