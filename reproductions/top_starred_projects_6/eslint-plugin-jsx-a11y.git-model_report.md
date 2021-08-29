# Model report for file:///tmp/top-repos-quality-repos-77014cct/eslint-plugin-jsx-a11y.git HEAD 125108849e4830a2aa98ae46039493900c45b0c7

### Dump

```json
{'created_at': '2021-08-29 21:00:57',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-4.15.0-135-generic-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.7 (default, Oct 22 2018, 11:32:17) [GCC 8.2.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '19.4 kB',
 'tags': [],
 'uuid': '60746410-fc45-486c-98ce-755f47d3c781',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-77014cct/eslint-plugin-jsx-a11y.git 125108849e4830a2aa98ae46039493900c45b0c7

# javascript
166 rules, avg.len. 9.2
## train
PPCR: 0.981907
### report
macro
{'f1-score': 0.8818392656460012,
 'precision': 0.9282905114768126,
 'recall': 0.8468664983089741,
 'support': 56984}
micro
{'f1-score': 0.9517759371051523,
 'precision': 0.9517759371051523,
 'recall': 0.9517759371051523,
 'support': 56984}
weighted
{'f1-score': 0.9501546642889067,
 'precision': 0.9510467740253162,
 'recall': 0.9517759371051523,
 'support': 56984}
### report_full
macro
{'f1-score': 0.8557079086859046,
 'precision': 0.9282905114768126,
 'recall': 0.8068361431953742,
 'support': 58034}
micro
{'f1-score': 0.9430871689648577,
 'precision': 0.9517759371051523,
 'recall': 0.9345556053348038,
 'support': 58034}
weighted
{'f1-score': 0.9401873840007369,
 'precision': 0.9506204308112778,
 'recall': 0.9345556053348038,
 'support': 58034}
## test
PPCR: 0.978687
### report
macro
{'f1-score': 0.8816523792338948,
 'precision': 0.9237280137579545,
 'recall': 0.8520164987634047,
 'support': 15062}
micro
{'f1-score': 0.9450936130659939,
 'precision': 0.9450936130659939,
 'recall': 0.9450936130659939,
 'support': 15062}
weighted
{'f1-score': 0.9436345080881082,
 'precision': 0.9450795289558558,
 'recall': 0.9450936130659939,
 'support': 15062}
### report_full
macro
{'f1-score': 0.8519846832495361,
 'precision': 0.9237280137579545,
 'recall': 0.8072338080289531,
 'support': 15390}
micro
{'f1-score': 0.934913962958098,
 'precision': 0.9450936130659939,
 'recall': 0.9249512670565302,
 'support': 15390}
weighted
{'f1-score': 0.9316923177978564,
 'precision': 0.9449798262318422,
 'recall': 0.9249512670565302,
 'support': 15390}
```

## javascript
### Summary
111 rules, avg.len. 9.0

| | |
|-|-|
|Min support|137|
|Max support|17070|
|Min confidence|0.9239864945411682|
|Max confidence|0.9998681545257568|

### Configuration

```json
{'feature_extractor': {'cutoff_label_support': 80,
                       'debug_parsing': False,
                       'label_composites': '<cut>',
                       'left_features': ['length',
                                         'diff_offset',
                                         'diff_col',
                                         'diff_line',
                                         'internal_type',
                                         'label',
                                         'reserved',
                                         'roles'],
                       'left_siblings_window': 5,
                       'no_labels_on_right': True,
                       'node_features': ['start_line', 'start_col'],
                       'parent_features': ['internal_type', 'roles'],
                       'parents_depth': 2,
                       'return_sibling_indices': False,
                       'right_features': ['length', 'internal_type', 'reserved', 'roles'],
                       'right_siblings_window': 5,
                       'select_features_number': 500,
                       'selected_features': '<cut>'},
 'line_length_limit': 500,
 'lines_ratio_train_trigger': 0.2,
 'lower_bound_instances': 500,
 'optimizer': {'base_model_name_categories': ['sklearn.ensemble.RandomForestClassifier',
                                              'sklearn.tree.DecisionTreeClassifier'],
               'cv': 3,
               'max_depth_categories': [None, 5, 10],
               'max_features_categories': [None, 'auto'],
               'min_samples_leaf_max': 120,
               'min_samples_leaf_min': 90,
               'min_samples_split_max': 240,
               'min_samples_split_min': 180,
               'n_iter': 50,
               'n_jobs': -1},
 'overall_size_limit': 5242880,
 'random_state': 42,
 'test_dataset_ratio': 0.2,
 'trainable_rules': {'attribute_similarity_threshold': 0.98,
                     'base_model_name': 'sklearn.ensemble.RandomForestClassifier',
                     'confidence_threshold': 0.8,
                     'min_samples_leaf': 90,
                     'min_samples_split': 180,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 1.000. Support: 3792.` |
| 2 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>⇒ y = '<br>Confidence: 1.000. Support: 3042.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -2.diff_col ≥ 4<br>	∧ +1.roles in {MAP}<br>	∧ ^1.internal_type not in {ObjectExpression}<br>⇒ y = ␣<br>Confidence: 0.989. Support: 4620.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ,<br>	∧ +1.roles not in {MAP}<br>	∧ +4.roles in {VALUE}<br>⇒ y = ⏎<br>Confidence: 0.999. Support: 1957.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ,<br>	∧ +1.roles not in {MAP}<br>	∧ +1.length ≤ 1<br>	∧ +4.roles not in {VALUE}<br>	∧ ^1.internal_type not in {CallExpression}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.971. Support: 629.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {,}<br>	∧ -2.roles in {MAP}<br>	∧ +1.roles not in {MAP}<br>⇒ y = ␣<br>Confidence: 0.988. Support: 2373.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {,}<br>	∧ -2.roles not in {MAP}<br>	∧ -3.length ≥ 3<br>	∧ +1.roles not in {MAP}<br>	∧ ^1.roles in {DECLARATION} and not in {FUNCTION}<br>⇒ y = ␣<br>Confidence: 0.993. Support: 1481.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {,}<br>	∧ -1.roles in {FUNCTION}<br>	∧ -2.roles not in {MAP}<br>	∧ -3.length ≤ 2<br>	∧ +1.roles not in {MAP}<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 206.` |
| 9 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {,}<br>	∧ -1.roles not in {FUNCTION}<br>	∧ -2.roles not in {MAP}<br>	∧ -3.length ≤ 2<br>	∧ +1.reserved = ,<br>	∧ +1.roles not in {MAP}<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.973. Support: 168.` |
| 10 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {,}<br>	∧ -1.roles not in {FUNCTION}<br>	∧ -3.diff_col ≤ 4<br>	∧ -3.label in {<space>}<br>	∧ -3.length ≤ 2<br>	∧ +1.reserved not in {,}<br>	∧ +1.roles not in {MAP}<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.980. Support: 178.` |
| 11 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ;<br>	∧ -2.roles not in {MAP}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {MAP}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.998. Support: 315.` |
| 12 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ;<br>	∧ -2.roles not in {MAP}<br>	∧ +1.reserved = import<br>	∧ +1.roles not in {MAP}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ⏎<br>Confidence: 0.998. Support: 251.` |
| 13 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {,, ;}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {MAP}<br>	∧ +2.reserved = ,<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.986. Support: 542.` |
| 14 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = const<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {MAP}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 421.` |
| 15 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = {<br>	∧ -2.roles not in {MAP}<br>	∧ -3.diff_col ≤ 4<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {MAP}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.996. Support: 339.` |
| 16 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {MAP}<br>	∧ +2.roles not in {MAP}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 436.` |
| 17 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = import<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {MAP}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 198.` |
| 18 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {,, ;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ +1.roles not in {MAP}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 182.` |
| 19 | `  -1.diff_col ≥ 4<br>	∧ -1.diff_offset ≥ 9<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {,, ;, const, import, {}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {MAP}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.992. Support: 179.` |
| 20 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {,, ;, const, import, {}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {MAP}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.970. Support: 13296.` |
| 21 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {,}<br>	∧ -2.roles in {MAP}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 4973.` |
| 22 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ,<br>	∧ -2.roles not in {MAP}<br>	∧ +4.roles in {VALUE}<br>⇒ y = ⏎<br>Confidence: 0.999. Support: 1905.` |
| 23 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ,<br>	∧ -2.roles not in {MAP}<br>	∧ +1.reserved = }<br>	∧ +4.roles not in {VALUE}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.998. Support: 319.` |
| 24 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ,<br>	∧ -2.roles not in {MAP}<br>	∧ +1.reserved = ]<br>	∧ +4.roles not in {VALUE}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.997. Support: 152.` |
| 25 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = {<br>	∧ -2.diff_col ≥ 3<br>	∧ -2.roles not in {MAP}<br>	∧ +1.length ≥ 2<br>⇒ y = ␣<br>Confidence: 0.977. Support: 2065.` |
| 26 | `  -1.diff_col ≥ 9<br>	∧ -1.diff_offset ≥ 3<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {,, {}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 2<br>⇒ y = '<br>Confidence: 0.998. Support: 209.` |
| 27 | `  -1.diff_col ≥ 9<br>	∧ -1.diff_offset ≥ 3<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {,, {}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ +2.roles in {LITERAL}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 145.` |
| 28 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ;<br>	∧ -2.roles not in {MAP}<br>	∧ +1.reserved = import<br>	∧ +1.length ≥ 2<br>⇒ y = ⏎<br>Confidence: 0.998. Support: 255.` |
| 29 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ -2.roles not in {MAP}<br>	∧ -3.length ≥ 3<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.974. Support: 398.` |
| 30 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {,}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>⇒ y = ␣<br>Confidence: 0.999. Support: 877.` |
| 31 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ;<br>	∧ -2.roles not in {MAP}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.999. Support: 340.` |
| 32 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {,, ;}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ +4.roles in {KEY}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 537.` |
| 33 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {,, ;}<br>	∧ -2.roles not in {MAP}<br>	∧ -2.length ≤ 1<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ +4.roles not in {KEY}<br>⇒ y = ␣<br>Confidence: 0.930. Support: 207.` |
| 34 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {,}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ +2.roles not in {MAP}<br>⇒ y = ␣<br>Confidence: 0.994. Support: 623.` |
| 35 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = if<br>	∧ -2.roles not in {MAP}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.length ≤ 1<br>⇒ y = ␣<br>Confidence: 0.997. Support: 146.` |
| 36 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {,, if}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.reserved not in {=, [, {, }}<br>	∧ +1.length ≤ 1<br>⇒ y = ∅<br>Confidence: 0.972. Support: 13887.` |
| 37 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ,<br>	∧ -2.roles not in {MAP}<br>	∧ +1.roles in {ARGUMENT}<br>	∧ +1.length ≥ 2<br>	∧ +4.roles not in {VALUE}<br>	∧ ^2.roles not in {INITIALIZATION}<br>⇒ y = ␣<br>Confidence: 0.924. Support: 296.` |
| 38 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ,<br>	∧ -2.roles not in {MAP}<br>	∧ +1.length ≤ 1<br>	∧ +4.roles not in {VALUE}<br>	∧ ^1.internal_type not in {CallExpression}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.965. Support: 588.` |
| 39 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = {<br>	∧ -2.diff_col ≤ 3<br>	∧ -2.roles not in {MAP}<br>	∧ -3.length ≤ 2<br>	∧ +1.length ≥ 2<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.929. Support: 793.` |
| 40 | `  -1.internal_type = CommentBlock<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {,, {}<br>	∧ -1.roles in {COMMENT}<br>	∧ -1.length ≥ 3<br>	∧ -2.roles not in {MAP}<br>	∧ +1.length ≥ 2<br>⇒ y = ∅<br>Confidence: 0.997. Support: 165.` |
| 41 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<newline>}<br>	∧ -1.reserved not in {,, {}<br>	∧ -1.length ≥ 3<br>	∧ -2.roles not in {MAP}<br>	∧ +1.length ≥ 2<br>⇒ y = '<br>Confidence: 0.998. Support: 265.` |
| 42 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ;<br>	∧ -1.length ≤ 2<br>	∧ -2.roles not in {MAP}<br>	∧ +1.reserved = import<br>	∧ +1.length ≥ 2<br>⇒ y = ⏎<br>Confidence: 0.998. Support: 242.` |
| 43 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ -1.length ≤ 2<br>	∧ -2.roles not in {MAP}<br>	∧ -3.length ≥ 3<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.979. Support: 405.` |
| 44 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ -1.length ≤ 2<br>	∧ -2.roles not in {MAP}<br>	∧ -3.diff_col = 0<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ +2.length ≥ 4<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 163.` |
| 45 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ -1.length ≤ 2<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ +2.length ≤ 3<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.929. Support: 3103.` |
| 46 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {,, ;}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved = ,<br>⇒ y = ␣<br>Confidence: 0.979. Support: 607.` |
| 47 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {,, ;}<br>	∧ -2.roles not in {MAP}<br>	∧ -2.length ≤ 1<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {,}<br>⇒ y = ␣<br>Confidence: 0.948. Support: 202.` |
| 48 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {,, if}<br>	∧ -2.diff_offset ≥ 3<br>	∧ -2.roles not in {MAP}<br>	∧ +1.reserved not in {=, [, {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {DECLARATION} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.975. Support: 569.` |
| 49 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {,, if}<br>	∧ -2.diff_offset ≤ 2<br>	∧ -2.roles not in {MAP}<br>	∧ -3.reserved = )<br>	∧ +1.reserved not in {=, [, {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {DECLARATION} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 164.` |
| 50 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {,, if}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.reserved not in {=, [, {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.984. Support: 12576.` |
| 51 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ,<br>	∧ +1.roles not in {MAP}<br>	∧ +1.length ≥ 2<br>	∧ +4.roles not in {VALUE}<br>	∧ +5.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {CallExpression}<br>⇒ y = ⏎<br>Confidence: 0.933. Support: 157.` |
| 52 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {,}<br>	∧ -2.roles not in {MAP}<br>	∧ -3.length ≥ 4<br>	∧ +1.roles not in {MAP}<br>	∧ ^1.roles in {DECLARATION} and not in {FUNCTION}<br>⇒ y = ␣<br>Confidence: 0.996. Support: 1437.` |
| 53 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {,}<br>	∧ -2.roles not in {MAP}<br>	∧ -3.length ≤ 3<br>	∧ +1.reserved = ,<br>	∧ +1.roles not in {MAP}<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.981. Support: 231.` |
| 54 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {,}<br>	∧ -1.roles in {FUNCTION}<br>	∧ -2.roles not in {MAP}<br>	∧ -3.length ≤ 3<br>	∧ +1.reserved not in {,}<br>	∧ +1.roles not in {MAP}<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 173.` |
| 55 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {,}<br>	∧ -1.roles not in {FUNCTION}<br>	∧ -3.diff_col ≤ 4<br>	∧ -3.label in {<space>}<br>	∧ -3.length ≤ 3<br>	∧ +1.reserved not in {,}<br>	∧ +1.roles not in {MAP}<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.957. Support: 196.` |
| 56 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.roles not in {MAP}<br>	∧ ^1.internal_type = CallExpression<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = '<br>Confidence: 0.972. Support: 511.` |
| 57 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = {<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {IMPORT, MAP}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.996. Support: 364.` |
| 58 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {,, ;, const, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {MAP}<br>	∧ +2.roles not in {MAP}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 473.` |
| 59 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {,, ;}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ +1.roles not in {MAP}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 217.` |
| 60 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = import<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {MAP}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 191.` |
| 61 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {,, ;, const, import, {}<br>	∧ -2.roles not in {MAP}<br>	∧ -5.diff_line ≤ 1<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {MAP}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.960. Support: 15962.` |
| 62 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ,<br>	∧ +1.roles not in {MAP}<br>	∧ +1.length ≥ 2<br>	∧ +4.reserved = ,<br>	∧ +4.roles not in {VALUE}<br>	∧ ^1.internal_type not in {CallExpression}<br>⇒ y = ⏎<br>Confidence: 0.953. Support: 159.` |
| 63 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -2.roles not in {MAP}<br>	∧ -3.length ≤ 3<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {MAP}<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 172.` |
| 64 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ><br>	∧ -2.roles not in {MAP}<br>	∧ -3.diff_col ≤ 4<br>	∧ -3.length ≤ 3<br>	∧ +1.reserved not in {)}<br>	∧ +1.roles not in {MAP}<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 170.` |
| 65 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {,, ;, const}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {MAP}<br>	∧ +2.roles not in {MAP}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 482.` |
| 66 | `  •••start_col ≥ 15<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = {<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {MAP}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.988. Support: 303.` |
| 67 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ,<br>	∧ -2.roles not in {MAP}<br>	∧ ^1.roles in {LIST}<br>⇒ y = ⏎<br>Confidence: 0.941. Support: 2270.` |
| 68 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ,<br>	∧ -2.roles not in {MAP}<br>	∧ +1.reserved = }<br>	∧ ^1.roles not in {LIST}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.998. Support: 236.` |
| 69 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ,<br>	∧ -2.roles not in {MAP}<br>	∧ +1.reserved = ]<br>	∧ ^1.roles not in {LIST}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.997. Support: 155.` |
| 70 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = {<br>	∧ -2.diff_col ≥ 8<br>	∧ -2.roles not in {MAP}<br>	∧ +1.length ≥ 2<br>⇒ y = ␣<br>Confidence: 0.985. Support: 2060.` |
| 71 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = {<br>	∧ -2.diff_col ≤ 7<br>	∧ -2.roles not in {MAP}<br>	∧ -3.diff_col ≤ 3<br>	∧ +1.length ≥ 2<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.931. Support: 785.` |
| 72 | `  -1.diff_offset ≥ 3<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {,, {}<br>	∧ -1.roles in {COMMENT}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.length ≥ 2<br>⇒ y = ∅<br>Confidence: 0.997. Support: 171.` |
| 73 | `  -1.diff_offset ≥ 3<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<newline>}<br>	∧ -1.reserved not in {,, {}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.length ≥ 2<br>⇒ y = '<br>Confidence: 0.998. Support: 200.` |
| 74 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {,, if}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {VARIABLE}<br>⇒ y = ␣<br>Confidence: 0.961. Support: 142.` |
| 75 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {,, if}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.974. Support: 13759.` |
| 76 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ,<br>	∧ -2.roles not in {MAP}<br>	∧ +1.reserved not in {], }}<br>	∧ +4.roles not in {VALUE}<br>	∧ +5.roles in {STRING}<br>⇒ y = ⏎<br>Confidence: 0.970. Support: 218.` |
| 77 | `  •••start_col ≥ 27<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ,<br>	∧ -2.roles not in {MAP}<br>	∧ -5.length ≤ 1<br>	∧ +1.reserved not in {], }}<br>	∧ +4.roles not in {VALUE}<br>	∧ +5.roles not in {STRING}<br>	∧ ^1.internal_type = CallExpression<br>⇒ y = ␣<br>Confidence: 0.991. Support: 703.` |
| 78 | `  •••start_col ≥ 27<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ,<br>	∧ -2.roles not in {MAP}<br>	∧ -5.length ≤ 1<br>	∧ +1.reserved not in {], }}<br>	∧ +4.roles not in {VALUE}<br>	∧ +4.length ≥ 13<br>	∧ +5.roles not in {STRING}<br>	∧ ^1.internal_type not in {CallExpression}<br>⇒ y = ␣<br>Confidence: 0.936. Support: 463.` |
| 79 | `  •••start_col ≥ 27<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ,<br>	∧ -2.roles not in {MAP}<br>	∧ -5.length ≤ 1<br>	∧ +1.reserved not in {], }}<br>	∧ +4.reserved = }<br>	∧ +4.roles not in {VALUE}<br>	∧ +4.length ≤ 12<br>	∧ +5.roles not in {STRING}<br>	∧ ^1.internal_type not in {CallExpression}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 155.` |
| 80 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {,, if}<br>	∧ -2.diff_offset ≤ 2<br>	∧ -2.roles not in {MAP}<br>	∧ +1.reserved = ><br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {DECLARATION} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 137.` |
| 81 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {,}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.reserved = =<br>	∧ +1.roles not in {MAP}<br>	∧ ^1.roles in {DECLARATION, FUNCTION}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 163.` |
| 82 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ><br>	∧ -2.roles not in {MAP}<br>	∧ +1.roles not in {MAP}<br>	∧ ^1.roles in {DECLARATION, FUNCTION}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 144.` |
| 83 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {,}<br>	∧ -2.roles not in {MAP}<br>	∧ -3.diff_col ≥ 4<br>	∧ -3.length ≥ 2<br>	∧ +1.roles not in {MAP}<br>	∧ ^1.roles in {DECLARATION} and not in {FUNCTION}<br>⇒ y = ␣<br>Confidence: 0.978. Support: 1542.` |
| 84 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {,}<br>	∧ -2.roles not in {MAP}<br>	∧ -3.diff_col ≥ 4<br>	∧ -3.length ≤ 1<br>	∧ +1.roles not in {MAP}<br>	∧ +3.length ≥ 6<br>	∧ ^1.roles in {DECLARATION} and not in {FUNCTION}<br>⇒ y = ␣<br>Confidence: 0.956. Support: 259.` |
| 85 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.roles not in {MAP}<br>	∧ ^1.roles in {CALL} and not in {DECLARATION}<br>⇒ y = '<br>Confidence: 0.976. Support: 481.` |
| 86 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = const<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {MAP}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 393.` |
| 87 | `  •••start_col ≥ 19<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = {<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {MAP}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.995. Support: 320.` |
| 88 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {,, ;, const, import}<br>	∧ -2.roles not in {MAP}<br>	∧ -3.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {MAP}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 210.` |
| 89 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {,, ;, const, import, {}<br>	∧ -2.roles not in {MAP}<br>	∧ -3.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), =, {, }}<br>	∧ +1.roles not in {MAP}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.roles in {EXPRESSION} and not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 171.` |
| 90 | `  -1.diff_col ≥ 4<br>	∧ -1.diff_offset ≥ 9<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {,, ;, const, import, {}<br>	∧ -2.roles not in {MAP}<br>	∧ -3.reserved not in {;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {MAP}<br>	∧ +1.length ≥ 2<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 190.` |
| 91 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {,, ;, const, import, {}<br>	∧ -2.roles not in {MAP}<br>	∧ -3.reserved not in {;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {MAP}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.979. Support: 12700.` |
| 92 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ,<br>	∧ -2.roles not in {MAP}<br>	∧ +1.length ≥ 2<br>	∧ +4.length ≥ 13<br>	∧ ^1.roles not in {LIST}<br>	∧ ^2.roles in {LITERAL}<br>⇒ y = ␣<br>Confidence: 0.952. Support: 491.` |
| 93 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ,<br>	∧ -2.roles not in {MAP}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles not in {LIST}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.990. Support: 451.` |
| 94 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = {<br>	∧ -2.diff_col ≥ 3<br>	∧ -2.roles not in {MAP}<br>⇒ y = ␣<br>Confidence: 0.980. Support: 2067.` |
| 95 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {,, {}<br>	∧ -2.roles not in {MAP}<br>	∧ -3.length ≥ 4<br>	∧ ^1.roles in {DECLARATION} and not in {FUNCTION}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 1398.` |
| 96 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {,, {}<br>	∧ -1.roles in {FUNCTION}<br>	∧ -2.roles not in {MAP}<br>	∧ -3.length ≤ 3<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 201.` |
| 97 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {,, {}<br>	∧ -1.roles not in {FUNCTION}<br>	∧ -2.roles not in {MAP}<br>	∧ -3.length ≤ 3<br>	∧ +1.reserved = ><br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 166.` |
| 98 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {,, {}<br>	∧ -1.roles not in {FUNCTION}<br>	∧ -2.roles not in {MAP}<br>	∧ -3.length ≤ 3<br>	∧ +1.reserved = ,<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.979. Support: 163.` |
| 99 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ;<br>	∧ -2.roles not in {MAP}<br>	∧ +1.reserved = }<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.999. Support: 380.` |
| 100 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ;<br>	∧ -2.roles not in {MAP}<br>	∧ +1.reserved = import<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ⏎<br>Confidence: 0.998. Support: 247.` |
| 101 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles in {EXPRESSION} and not in {DECLARATION}<br>⇒ y = '<br>Confidence: 0.927. Support: 762.` |
| 102 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +2.reserved = ,<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.990. Support: 567.` |
| 103 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +2.roles not in {MAP}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 451.` |
| 104 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = const<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 370.` |
| 105 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = import<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 237.` |
| 106 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {,, ;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 221.` |
| 107 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {,, ;, const, import, {}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.944. Support: 17070.` |
| 108 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {,, {}<br>	∧ -1.roles in {BLOCK, COMMENT}<br>	∧ -1.length ≥ 3<br>	∧ -2.roles not in {MAP}<br>	∧ +1.length ≥ 2<br>⇒ y = ∅<br>Confidence: 0.997. Support: 156.` |
| 109 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {,, {}<br>	∧ -1.roles not in {COMMENT}<br>	∧ -1.length ≥ 3<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.940. Support: 1835.` |
| 110 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {,, if}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type = VariableDeclarator<br>⇒ y = ␣<br>Confidence: 0.934. Support: 145.` |
| 111 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {,, if}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>⇒ y = ∅<br>Confidence: 0.974. Support: 13863.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 8.99099099099099, "max_conf": 0.9998681545257568, "max_support": 17070, "min_conf": 0.9239864945411682, "min_support": 137, "num_rules": 111}}
```
</details>
