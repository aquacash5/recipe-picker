# Recipe Picker

Pick recipes from a CSV of recipes and tags.

## CLI

```text
Recipe Picker

Pick recipes from a CSV of recipes and tags

Usage: recipe-picker --input <INPUT> <COMMAND>

Commands:
  tags    Print out tags
  total   Print out the total number of recipes
  sample  Get recipe samples
  help    Print this message or the help of the given subcommand(s)

Options:
  -i, --input <INPUT>
          File with all of the recipes
          
          [env: RECIPE_PICKER_INPUT=<path to recipe list>]

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

### sample

```text
Get recipe samples

Usage: recipe-picker --input <INPUT> sample [OPTIONS] [-- <TAGS>...]

Arguments:
  [TAGS]...  Tags to include or exclude

Options:
  -r, --results <RESULTS>  How many results do you want to get from the recipes [default: 5]
  -h, --help               Print help
```

#### Basic search

```bash
recipe-picker sample -- main soup
```

```text
101 Thai Dishes to Try Before You Die: Beef boat noodle soup
The Book of Jewish Food              : Chicken Soup
Wagamama Feed Your Soul              : chicken ramen
Wagamama Feed Your Soul              : miso ramen
Wagamama Feed Your Soul              : shirodashi ramen
```

#### Change sample size

```bash
recipe-picker sample -r 10 -- dessert
```

```text
Chinese Takeaway Cookbook: Steamed Bao Buns
Comfort and Joy          : 1 dozen rummy oatmeal cookies
Comfort and Joy          : double chocolate chip cookies
Comfort and Joy          : Pecan Pie
Smitten Kitchen Keepers  : Better than classic pound cake
Smitten Kitchen Keepers  : chocolate chip cookies with salted walnut brittle
Smitten Kitchen Keepers  : chocolate peanut butter cup cookies
Smitten Kitchen Keepers  : whole lemon poppy seed cake
The Book of Jewish Food  : Apple Strudel
The XXL BBQ Cookbook     : bbq pineapples and peaches
```

#### Negative tag

```bash
recipe-picker sample -- main -beef
```

```text
Barbecue Sauces Rubs and Marinades: righteous ribs
Korean Cookbook for Beginners     : boiled park
The Food Lab                      : basic pan seared pork chops
The Food Lab                      : pan seared pork chips with apple and cider sauce
Wagamama Feed Your Soul           : tebasaki chicken wings
```

## Fields

The following are acceptable fields for the CSV:

| Header   | Format   | Required |
| -------- | -------- | -------- |
| Recipe   | text     | Yes      |
| Book     | text     | Yes      |
| Category | text     | No       |
| Type     | text     | No       |
| Tags     | list (,) | No       |

### Example

```csv
Recipe,Book,Category,Type,Tags
Vegetarian mini spring rolls ,Chinese Takeaway Cookbook,side,veg,
Satay Chicken Skewers,Chinese Takeaway Cookbook,main,chicken,
Sweet Chinese BBQ rips,Chinese Takeaway Cookbook,main,pork,
Classic Pancake Rolls,Chinese Takeaway Cookbook,side,veg,
herbes de provence,Barbecue Sauces Rubs and Marinades,main,,spices
mediterranean herb rub,Barbecue Sauces Rubs and Marinades,main,,spices
dakdoritang (korean spicy chicken stew),Korean Cookbook for Beginners,main,chicken,"soup,stew,favorite"
dakjjim (korean chicken stew),Korean Cookbook for Beginners,main,chicken,"soup,stew"
```
