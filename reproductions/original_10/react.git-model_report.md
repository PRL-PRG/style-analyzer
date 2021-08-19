# Model report for file:///tmp/top-repos-quality-repos-c25srffy/react.git HEAD 5634ed16aaba4a507844f0502953abcf40e3a165

### Dump

```json
{'created_at': '2021-08-18 08:10:10',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-5.4.0-80-generic-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.9 (default, Jan 26 2021, 15:33:00) [GCC 8.4.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '24.8 kB',
 'tags': [],
 'uuid': 'ff695443-364b-46ba-bbd0-5b1e890257db',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-c25srffy/react.git 5634ed16aaba4a507844f0502953abcf40e3a165

# javascript
148 rules, avg.len. 10.8
## train
PPCR: 0.955757
### report
macro
{'f1-score': 0.6440227557453749,
 'precision': 0.7105595680795925,
 'recall': 0.6084845660817259,
 'support': 596984}
micro
{'f1-score': 0.9680376693512724,
 'precision': 0.9680376693512724,
 'recall': 0.9680376693512724,
 'support': 596984}
weighted
{'f1-score': 0.9666397085414921,
 'precision': 0.9678814757047726,
 'recall': 0.9680376693512724,
 'support': 596984}
### report_full
macro
{'f1-score': 0.6074109448498922,
 'precision': 0.7105595680795925,
 'recall': 0.5582680442329798,
 'support': 624619}
micro
{'f1-score': 0.9461388028680349,
 'precision': 0.9680376693512724,
 'recall': 0.9252088072889233,
 'support': 624619}
weighted
{'f1-score': 0.9412415803912288,
 'precision': 0.966438819375155,
 'recall': 0.9252088072889233,
 'support': 624619}
## test
PPCR: 0.953354
### report
macro
{'f1-score': 0.6040812864294102,
 'precision': 0.6836914087972891,
 'recall': 0.5654417634072846,
 'support': 118458}
micro
{'f1-score': 0.9537895287781323,
 'precision': 0.9537895287781323,
 'recall': 0.9537895287781323,
 'support': 118458}
weighted
{'f1-score': 0.951041292437686,
 'precision': 0.9539555683445106,
 'recall': 0.9537895287781323,
 'support': 118458}
### report_full
macro
{'f1-score': 0.567381564662508,
 'precision': 0.6836914087972891,
 'recall': 0.516253845969774,
 'support': 124254}
micro
{'f1-score': 0.9310128877022974,
 'precision': 0.9537895287781323,
 'recall': 0.909298694609429,
 'support': 124254}
weighted
{'f1-score': 0.9238631745734069,
 'precision': 0.9519821702805804,
 'recall': 0.909298694609429,
 'support': 124254}
```

## javascript
### Summary
148 rules, avg.len. 10.8

| | |
|-|-|
|Min support|128|
|Max support|46941|
|Min confidence|0.80078125|
|Max confidence|0.9999798536300659|

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
               'min_samples_leaf_max': 130,
               'min_samples_leaf_min': 90,
               'min_samples_split_max': 240,
               'min_samples_split_min': 180,
               'n_iter': 50,
               'n_jobs': -1},
 'overall_size_limit': 5242880,
 'random_state': 42,
 'test_dataset_ratio': 0.2,
 'trainable_rules': {'attribute_similarity_threshold': 0.98,
                     'base_model_name': 'sklearn.tree.DecisionTreeClassifier',
                     'confidence_threshold': 0.8,
                     'min_samples_leaf': 128,
                     'min_samples_split': 239,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -1.internal_type = StringLiteral<br>	∧ -2.reserved = =<br>⇒ y = "<br>Confidence: 1.000. Support: 1084.` |
| 2 | `  -1.internal_type = StringLiteral<br>	∧ -2.reserved not in {=}<br>⇒ y = '<br>Confidence: 0.991. Support: 19202.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = .<br>	∧ +1.length ≥ 2<br>⇒ y = ∅<br>Confidence: 1.000. Support: 24787.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -2.label in {<space>}<br>	∧ +1.roles in {MAP}<br>	∧ +1.length ≥ 2<br>	∧ +4.reserved = ,<br>	∧ ^2.roles not in {MAP}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.889. Support: 616.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -2.label in {<space>}<br>	∧ +1.roles in {MAP}<br>	∧ +1.length ≥ 2<br>	∧ +3.roles in {VALUE}<br>	∧ +4.reserved not in {,}<br>⇒ y = ∅<br>Confidence: 0.994. Support: 750.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -2.label in {<space>}<br>	∧ +1.roles in {MAP}<br>	∧ +1.length ≥ 2<br>	∧ +2.reserved = }<br>	∧ +3.roles not in {VALUE}<br>	∧ +4.reserved not in {,}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 180.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -2.label in {<space>}<br>	∧ +1.roles in {MAP}<br>	∧ +1.length ≥ 2<br>	∧ +2.reserved not in {}}<br>	∧ +3.roles not in {VALUE}<br>	∧ +4.reserved not in {,}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.855. Support: 438.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -2.label in {<space>}<br>	∧ +1.roles in {IMPORT} and not in {MAP}<br>	∧ +1.length ≥ 2<br>	∧ +4.reserved = ,<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.864. Support: 188.` |
| 9 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -2.label in {<space>}<br>	∧ +1.roles in {IMPORT} and not in {MAP}<br>	∧ +1.length ≥ 2<br>	∧ +4.reserved not in {,}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 332.` |
| 10 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -2.label in {<space>}<br>	∧ +1.roles not in {IMPORT, MAP}<br>	∧ +1.length ≥ 2<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.994. Support: 12966.` |
| 11 | `  •••start_col ≥ 7<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -2.label not in {<space>}<br>	∧ +1.length ≥ 2<br>	∧ +4.reserved not in {,}<br>	∧ +4.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.942. Support: 2536.` |
| 12 | `  •••start_col ≤ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -2.label not in {<space>}<br>	∧ -4.diff_col ≤ 6<br>	∧ +1.length ≥ 2<br>	∧ +3.roles in {VALUE}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.831. Support: 359.` |
| 13 | `  •••start_col ≤ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -2.label not in {<space>}<br>	∧ +1.length ≥ 2<br>	∧ +3.roles not in {VALUE}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.994. Support: 250.` |
| 14 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 2<br>⇒ y = ␣<br>Confidence: 0.987. Support: 3901.` |
| 15 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {., :, {}<br>	∧ -2.label in {<space>}<br>	∧ -3.diff_col ≥ 4<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 2<br>⇒ y = ␣<br>Confidence: 0.977. Support: 2286.` |
| 16 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {., :, {}<br>	∧ -2.label in {<space>}<br>	∧ -3.diff_col ≤ 3<br>	∧ -4.roles in {RIGHT}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 2<br>⇒ y = ⏎<br>Confidence: 0.858. Support: 687.` |
| 17 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {., :, {}<br>	∧ -2.label in {<space>}<br>	∧ -3.diff_col ≤ 3<br>	∧ -4.roles in {LEFT} and not in {RIGHT}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 2<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.846. Support: 537.` |
| 18 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -2.label not in {<space>}<br>	∧ -5.label in {<newline>}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 2<br>⇒ y = ⏎<br>Confidence: 0.920. Support: 1123.` |
| 19 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -2.label not in {<space>}<br>	∧ -5.label not in {<newline>}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.roles in {MAP}<br>	∧ +1.length ≥ 2<br>⇒ y = ⏎<br>Confidence: 0.936. Support: 241.` |
| 20 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -2.label not in {<space>}<br>	∧ -3.internal_type = StringLiteral<br>	∧ -5.label not in {<newline>}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.roles not in {MAP}<br>	∧ +1.length ≤ 28<br>⇒ y = ␣<br>Confidence: 0.999. Support: 341.` |
| 21 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -2.label not in {<space>}<br>	∧ -3.internal_type not in {StringLiteral}<br>	∧ -4.internal_type = Identifier<br>	∧ -5.label not in {<newline>}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.roles not in {MAP}<br>	∧ +1.length ≤ 28<br>⇒ y = ␣<br>Confidence: 0.969. Support: 175.` |
| 22 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = =<br>	∧ -2.label not in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 2<br>⇒ y = "<br>Confidence: 0.999. Support: 840.` |
| 23 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ., :, =, {}<br>	∧ -2.label not in {<space>}<br>	∧ -3.reserved = .<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 2<br>	∧ +2.reserved = )<br>⇒ y = '<br>Confidence: 0.985. Support: 1701.` |
| 24 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ., :, =, {}<br>	∧ -2.label not in {<space>}<br>	∧ -3.reserved = .<br>	∧ -5.label in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 2<br>	∧ +2.reserved not in {)}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.927. Support: 612.` |
| 25 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ., :, =, {}<br>	∧ -2.label not in {<space>}<br>	∧ -3.reserved = .<br>	∧ -5.label not in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 30<br>	∧ +2.reserved not in {)}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.955. Support: 452.` |
| 26 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ., =, {}<br>	∧ -2.label in {<newline>} and not in {<space>}<br>	∧ -3.reserved not in {.}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 2<br>⇒ y = ␣<br>Confidence: 0.898. Support: 358.` |
| 27 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ., :, =, {}<br>	∧ -1.roles in {COMMENT}<br>	∧ -2.label not in {<newline>, <space>}<br>	∧ -3.reserved not in {.}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 2<br>⇒ y = ⏎<br>Confidence: 0.993. Support: 223.` |
| 28 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ., :, =, {}<br>	∧ -2.label not in {<newline>, <space>}<br>	∧ -3.reserved not in {.}<br>	∧ -4.diff_line ≥ 1<br>	∧ -4.reserved = .<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 2<br>	∧ +5.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 0.955. Support: 190.` |
| 29 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ., :, =, {}<br>	∧ -2.label not in {<newline>, <space>}<br>	∧ -3.reserved not in {.}<br>	∧ -4.reserved = .<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 2<br>	∧ +5.internal_type not in {StringLiteral}<br>⇒ y = '<br>Confidence: 0.974. Support: 1819.` |
| 30 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ., :, =, {}<br>	∧ -2.label not in {<newline>, <space>}<br>	∧ -3.reserved not in {.}<br>	∧ -4.reserved not in {.}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 2<br>⇒ y = '<br>Confidence: 0.983. Support: 13682.` |
| 31 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ +2.length ≥ 13<br>	∧ +4.roles in {FUNCTION}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.957. Support: 152.` |
| 32 | `  •••start_col ≥ 35<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ +2.length ≤ 12<br>	∧ +4.roles in {FUNCTION}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.801. Support: 128.` |
| 33 | `  •••start_col ≤ 46<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ +3.length ≤ 13<br>	∧ +4.reserved = ,<br>⇒ y = ∅<br>Confidence: 0.845. Support: 1032.` |
| 34 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ +2.reserved = )<br>	∧ +4.reserved not in {,}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 6709.` |
| 35 | `  •••start_col ≤ 54<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 3<br>	∧ +2.reserved not in {)}<br>	∧ +4.reserved not in {,}<br>⇒ y = ∅<br>Confidence: 0.930. Support: 9184.` |
| 36 | `  •••start_col ≥ 7<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ -3.internal_type = NumericLiteral<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ +2.roles in {COMMENT}<br>	∧ +2.length ≤ 13<br>⇒ y = ␣<br>Confidence: 0.968. Support: 139.` |
| 37 | `  •••start_col ≥ 26<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {export}<br>	∧ +1.length ≥ 6<br>	∧ +3.roles not in {LITERAL}<br>⇒ y = ⏎<br>Confidence: 0.876. Support: 6206.` |
| 38 | `  •••start_col ≤ 25<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ -4.diff_offset ≤ 13<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {export}<br>	∧ +1.length ≥ 6<br>	∧ +3.roles not in {LITERAL}<br>⇒ y = ⏎<br>Confidence: 0.858. Support: 425.` |
| 39 | `  •••start_col ≥ 15<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ -2.roles in {EXPRESSION}<br>	∧ -3.length ≤ 6<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≤ 5<br>	∧ +2.roles not in {COMMENT}<br>⇒ y = ⏎<br>Confidence: 0.833. Support: 2479.` |
| 40 | `  •••start_col ≥ 15<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ -2.roles not in {EXPRESSION}<br>	∧ -3.length ≤ 6<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≤ 5<br>	∧ +2.roles not in {COMMENT}<br>	∧ +3.internal_type = Identifier<br>	∧ ^2.roles not in {ARGUMENT}<br>⇒ y = ⏎<br>Confidence: 0.870. Support: 571.` |
| 41 | `  •••start_col ≥ 15<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ -2.roles not in {EXPRESSION}<br>	∧ -3.length ≤ 6<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≤ 5<br>	∧ +2.roles not in {COMMENT}<br>	∧ +3.internal_type not in {Identifier}<br>	∧ +4.roles in {CALL}<br>	∧ ^2.roles not in {ARGUMENT}<br>⇒ y = ⏎<br>Confidence: 0.889. Support: 356.` |
| 42 | `  •••start_col ≤ 14<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ -2.label in {'}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ +2.roles not in {COMMENT}<br>⇒ y = ⏎⏎<br>Confidence: 0.997. Support: 189.` |
| 43 | `  •••start_col ≤ 14<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ -2.label not in {'}<br>	∧ -3.diff_col ≥ 4<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ +2.roles not in {COMMENT}<br>	∧ +5.internal_type = Identifier<br>⇒ y = ⏎<br>Confidence: 0.869. Support: 415.` |
| 44 | `  •••start_col ≤ 14<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ -2.label not in {'}<br>	∧ -3.diff_col ≤ 3<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≤ 3<br>	∧ +2.roles not in {COMMENT}<br>⇒ y = ⏎⏎<br>Confidence: 0.841. Support: 173.` |
| 45 | `  •••start_col ≤ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ -3.diff_col ≤ 5<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>⇒ y = ⏎⏎<br>Confidence: 0.953. Support: 1485.` |
| 46 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ., ;, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {COMMENT}<br>	∧ +1.length ≥ 5<br>⇒ y = ∅<br>Confidence: 1.000. Support: 7670.` |
| 47 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ., ;, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {COMMENT}<br>	∧ +1.length ≤ 4<br>⇒ y = ⏎<br>Confidence: 0.840. Support: 134.` |
| 48 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ., ;, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = JSXElement<br>⇒ y = ∅<br>Confidence: 1.000. Support: 5633.` |
| 49 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {(, ., ;, {}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>⇒ y = ⏎<br>Confidence: 0.951. Support: 6771.` |
| 50 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -5.diff_line = 0<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ +2.reserved = )<br>	∧ ^1.internal_type not in {JSXElement}<br>⇒ y = ␣<br>Confidence: 0.977. Support: 2078.` |
| 51 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -5.diff_line ≥ 1<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ +2.reserved not in {)}<br>	∧ ^1.internal_type not in {JSXElement}<br>⇒ y = ⏎<br>Confidence: 0.985. Support: 4759.` |
| 52 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -2.roles in {ARGUMENT}<br>	∧ -5.diff_line = 0<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ +2.reserved not in {)}<br>	∧ ^1.internal_type not in {JSXElement}<br>⇒ y = ␣<br>Confidence: 1.000. Support: 1155.` |
| 53 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -2.roles not in {ARGUMENT}<br>	∧ -3.diff_col ≥ 5<br>	∧ -5.diff_line = 0<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ +2.reserved not in {), ,}<br>	∧ +4.reserved = ,<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^2.roles in {MAP}<br>⇒ y = ⏎<br>Confidence: 0.924. Support: 257.` |
| 54 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -2.roles not in {ARGUMENT}<br>	∧ -5.diff_line = 0<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ +2.reserved not in {), ,}<br>	∧ +4.reserved = ,<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^2.roles not in {MAP}<br>⇒ y = ⏎<br>Confidence: 0.952. Support: 2853.` |
| 55 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -2.roles not in {ARGUMENT}<br>	∧ -5.diff_line = 0<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ +2.reserved not in {)}<br>	∧ +4.reserved not in {,}<br>	∧ +5.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles in {LIST}<br>⇒ y = ␣<br>Confidence: 0.838. Support: 157.` |
| 56 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -2.roles not in {ARGUMENT}<br>	∧ -5.diff_line = 0<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ +2.reserved not in {)}<br>	∧ +4.reserved not in {,}<br>	∧ +5.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {JSXElement}<br>⇒ y = ⏎<br>Confidence: 0.849. Support: 1850.` |
| 57 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -2.roles not in {ARGUMENT}<br>	∧ -5.diff_line = 0<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ +2.reserved not in {)}<br>	∧ +3.roles in {VALUE}<br>	∧ +4.reserved not in {,}<br>	∧ +5.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {JSXElement}<br>⇒ y = ␣<br>Confidence: 0.952. Support: 764.` |
| 58 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -2.label not in {'}<br>	∧ -2.roles not in {ARGUMENT}<br>	∧ -5.diff_line = 0<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {CALL}<br>	∧ +1.length ≥ 2<br>	∧ +2.reserved not in {), :}<br>	∧ +4.reserved not in {,}<br>	∧ +5.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {JSXElement}<br>⇒ y = ⏎<br>Confidence: 0.849. Support: 328.` |
| 59 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -2.roles not in {ARGUMENT}<br>	∧ -5.diff_line = 0<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {CALL, COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ +2.reserved not in {), :}<br>	∧ +4.reserved not in {,}<br>	∧ +5.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {JSXElement}<br>⇒ y = ␣<br>Confidence: 0.855. Support: 1007.` |
| 60 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = }<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = IfStatement<br>⇒ y = ␣<br>Confidence: 0.999. Support: 655.` |
| 61 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = }<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {IfStatement, JSXElement}<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 539.` |
| 62 | `  •••start_col ≥ 5<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = }<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {IfStatement}<br>	∧ ^1.roles in {LITERAL} and not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 364.` |
| 63 | `  •••start_col ≥ 5<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = }<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {IfStatement, JSXElement}<br>	∧ ^1.roles in {BODY} and not in {DECLARATION, LITERAL}<br>	∧ ^2.internal_type not in {IfStatement}<br>⇒ y = ⏎<br>Confidence: 0.900. Support: 1448.` |
| 64 | `  •••start_col ≥ 5<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = }<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ +2.reserved = {<br>	∧ ^1.internal_type not in {IfStatement, JSXElement}<br>	∧ ^1.roles not in {BODY, DECLARATION, LITERAL}<br>⇒ y = ␣<br>Confidence: 0.971. Support: 154.` |
| 65 | `  •••start_col ≥ 5<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = }<br>	∧ -4.diff_offset ≥ 12<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 3<br>	∧ ^1.internal_type not in {IfStatement, JSXElement}<br>	∧ ^1.roles not in {BODY, DECLARATION, LITERAL, SCOPE}<br>⇒ y = ⏎<br>Confidence: 0.918. Support: 285.` |
| 66 | `  •••start_col ≤ 5<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = }<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = Program<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ⏎⏎<br>Confidence: 0.981. Support: 877.` |
| 67 | `  •••start_col ≤ 5<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = }<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {IfStatement, JSXElement}<br>	∧ ^1.roles not in {DECLARATION}<br>	∧ ^2.internal_type = FunctionExpression<br>⇒ y = ⏎⏎<br>Confidence: 0.943. Support: 325.` |
| 68 | `  •••start_col ≤ 5<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = }<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ +2.length ≤ 9<br>	∧ ^1.internal_type not in {IfStatement, JSXElement}<br>	∧ ^1.roles not in {BODY, DECLARATION}<br>	∧ ^2.internal_type not in {FunctionExpression}<br>⇒ y = ␣<br>Confidence: 0.848. Support: 128.` |
| 69 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = <<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 2267.` |
| 70 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ,, ;, <, {, }}<br>	∧ -1.roles in {ARGUMENT}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>⇒ y = ∅<br>Confidence: 0.998. Support: 1461.` |
| 71 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ,, ;, <, {, }}<br>	∧ -1.roles not in {ARGUMENT}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {LITERAL}<br>⇒ y = ∅<br>Confidence: 0.966. Support: 1316.` |
| 72 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ,, ;, <, {, }}<br>	∧ -1.roles not in {ARGUMENT}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = ∅<br>Confidence: 0.938. Support: 522.` |
| 73 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ,, ., ;, {, }}<br>	∧ -1.roles not in {ARGUMENT}<br>	∧ -2.roles in {CONDITION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = SwitchStatement<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = ⏎<br>Confidence: 0.998. Support: 228.` |
| 74 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ,, ., ;, }}<br>	∧ -1.roles not in {ARGUMENT}<br>	∧ -2.roles in {CONDITION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.998. Support: 285.` |
| 75 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ,, ;, <, {, }}<br>	∧ -1.roles not in {ARGUMENT}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = MemberExpression<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 355.` |
| 76 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ,, ;, <, {, }}<br>	∧ -1.roles in {COMMENT} and not in {ARGUMENT}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 338.` |
| 77 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ,, ;, <, {, }}<br>	∧ -1.roles not in {ARGUMENT}<br>	∧ -3.reserved = const<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = ∅<br>Confidence: 0.882. Support: 360.` |
| 78 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ,, ;, <, {, }}<br>	∧ -1.roles not in {ARGUMENT}<br>	∧ -3.diff_offset ≥ 5<br>	∧ -3.reserved not in {const}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ +3.internal_type = JSXIdentifier<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = ∅<br>Confidence: 0.887. Support: 137.` |
| 79 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ,, ., ;, <, {, }}<br>	∧ -1.roles not in {ARGUMENT}<br>	∧ -3.diff_offset ≥ 5<br>	∧ -4.label in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ +3.internal_type not in {JSXIdentifier}<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>	∧ ^1.roles in {CALL} and not in {LITERAL}<br>⇒ y = ␣<br>Confidence: 0.958. Support: 1010.` |
| 80 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ,, ., ;, <, {, }}<br>	∧ -1.roles not in {ARGUMENT}<br>	∧ -3.diff_offset ≥ 5<br>	∧ -4.label not in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ +3.internal_type not in {JSXIdentifier}<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>	∧ ^1.roles in {CALL, OPERATOR} and not in {LITERAL}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 143.` |
| 81 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ,, ., ;, <, {, }}<br>	∧ -1.roles not in {ARGUMENT}<br>	∧ -3.diff_offset ≥ 5<br>	∧ -3.roles in {RIGHT}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ +3.internal_type not in {JSXIdentifier}<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>	∧ ^1.roles in {CONDITION} and not in {CALL, LITERAL}<br>⇒ y = ␣<br>Confidence: 0.833. Support: 285.` |
| 82 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ,, ., ;, <, {, }}<br>	∧ -1.roles not in {ARGUMENT}<br>	∧ -3.diff_offset ≥ 5<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ +3.internal_type not in {JSXIdentifier}<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>	∧ ^1.roles not in {CALL, LITERAL}<br>	∧ ^2.internal_type = JSXElement<br>⇒ y = ␣<br>Confidence: 0.810. Support: 1978.` |
| 83 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ,, ., ;, <, {, }}<br>	∧ -1.roles not in {ARGUMENT}<br>	∧ -3.diff_offset ≥ 5<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {EXPRESSION} and not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ +3.internal_type not in {JSXIdentifier}<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>	∧ ^1.roles in {SWITCH} and not in {CALL, LITERAL}<br>	∧ ^2.internal_type not in {JSXElement}<br>⇒ y = ␣<br>Confidence: 0.962. Support: 810.` |
| 84 | `  -1.diff_col ≤ 25<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ,, ., ;, <, {, }}<br>	∧ -1.roles not in {ARGUMENT}<br>	∧ -3.diff_offset ≥ 5<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ +2.roles not in {COMMENT}<br>	∧ +3.internal_type not in {JSXIdentifier}<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>	∧ ^1.roles in {AND} and not in {CALL, LITERAL}<br>	∧ ^2.internal_type not in {JSXElement}<br>⇒ y = ␣<br>Confidence: 0.915. Support: 1445.` |
| 85 | `  -1.diff_col ≤ 25<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ,, ., ;, <, {, }}<br>	∧ -1.roles not in {ARGUMENT}<br>	∧ -3.diff_offset ≥ 5<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ +2.roles not in {COMMENT}<br>	∧ +2.length ≥ 2<br>	∧ +3.internal_type not in {JSXIdentifier}<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>	∧ ^1.roles not in {CALL, LITERAL}<br>	∧ ^2.internal_type not in {JSXElement}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 1642.` |
| 86 | `  -1.diff_col ≤ 8<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ,, ., ;, <, {, }}<br>	∧ -1.roles not in {ARGUMENT}<br>	∧ -3.diff_offset ≥ 5<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ +2.roles not in {COMMENT}<br>	∧ +3.internal_type not in {JSXIdentifier}<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>	∧ ^1.roles not in {CALL, LITERAL}<br>	∧ ^2.internal_type not in {JSXElement}<br>⇒ y = ␣<br>Confidence: 0.990. Support: 36829.` |
| 87 | `  -1.diff_col ≤ 1<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ,, ;, <, {, }}<br>	∧ -1.roles not in {ARGUMENT}<br>	∧ -2.reserved = (<br>	∧ -3.diff_offset ≤ 4<br>	∧ -3.reserved not in {:, const}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ +3.reserved not in {{}<br>	∧ +3.roles in {IDENTIFIER}<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 139.` |
| 88 | `  -1.diff_col ≤ 1<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ,, ., ;, <, {, }}<br>	∧ -1.roles not in {ARGUMENT}<br>	∧ -3.diff_offset ≤ 4<br>	∧ -3.reserved not in {:}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ +3.reserved not in {{}<br>	∧ +3.roles not in {IDENTIFIER}<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = ␣<br>Confidence: 0.899. Support: 2661.` |
| 89 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ,, ., ;, <, {, }}<br>	∧ -1.roles not in {ARGUMENT}<br>	∧ -2.roles in {BLOCK}<br>	∧ -3.length ≥ 1<br>	∧ -5.diff_offset ≤ 2<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = ⏎⏎<br>Confidence: 0.921. Support: 247.` |
| 90 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ,, ;, <, {, }}<br>	∧ -1.roles not in {ARGUMENT}<br>	∧ -3.length ≤ 1<br>	∧ -5.diff_offset ≤ 2<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 608.` |
| 91 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.internal_type = JSXIdentifier<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>⇒ y = ∅<br>Confidence: 1.000. Support: 1669.` |
| 92 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.internal_type not in {JSXIdentifier}<br>	∧ -3.roles in {ARGUMENT}<br>	∧ -5.label in {<+space>}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>⇒ y = ⏎<br>Confidence: 0.999. Support: 986.` |
| 93 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.internal_type not in {JSXIdentifier}<br>	∧ -5.label not in {<+space>}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {LIST}<br>⇒ y = ⏎<br>Confidence: 0.939. Support: 612.` |
| 94 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -2.internal_type not in {JSXIdentifier}<br>	∧ -5.label not in {<+space>}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {LIST}<br>⇒ y = ∅<br>Confidence: 0.964. Support: 549.` |
| 95 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ -2.internal_type not in {JSXIdentifier}<br>	∧ -5.label not in {<+space>}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type = JSXElement<br>	∧ ^1.roles not in {LIST}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 252.` |
| 96 | `  •••start_col ≥ 32<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -2.internal_type not in {JSXIdentifier}<br>	∧ -3.diff_offset ≤ 46<br>	∧ -4.roles not in {KEY}<br>	∧ -5.label not in {<+space>}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {LIST}<br>⇒ y = ␣<br>Confidence: 0.970. Support: 1376.` |
| 97 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ,, {}<br>	∧ -2.internal_type not in {JSXIdentifier}<br>	∧ -3.diff_offset ≤ 46<br>	∧ -4.roles not in {KEY}<br>	∧ -5.label not in {<+space>}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {LIST}<br>⇒ y = ␣<br>Confidence: 0.993. Support: 15232.` |
| 98 | `  -1.internal_type = JSXIdentifier<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>⇒ y = ∅<br>Confidence: 1.000. Support: 2618.` |
| 99 | `  -1.diff_offset ≥ 2<br>	∧ -1.internal_type not in {JSXIdentifier, StringLiteral}<br>	∧ -4.reserved = .<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>⇒ y = ␣<br>Confidence: 0.999. Support: 383.` |
| 100 | `  -1.internal_type not in {JSXIdentifier, StringLiteral}<br>	∧ -4.reserved not in {.}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>⇒ y = ␣<br>Confidence: 0.995. Support: 15506.` |
| 101 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>⇒ y = ∅<br>Confidence: 0.988. Support: 4668.` |
| 102 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.817. Support: 490.` |
| 103 | `  •••start_col ≥ 58<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type = CallExpression<br>	∧ ^1.roles not in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.832. Support: 134.` |
| 104 | `  •••start_col ≤ 57<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type = CallExpression<br>	∧ ^1.roles not in {INCOMPLETE}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.899. Support: 572.` |
| 105 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles not in {INCOMPLETE}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.971. Support: 15303.` |
| 106 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = if<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.length ≤ 1<br>⇒ y = ␣<br>Confidence: 1.000. Support: 4946.` |
| 107 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {CALL, QUALIFIED}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.967. Support: 168.` |
| 108 | `  •••start_col ≥ 31<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {CALL} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.936. Support: 1676.` |
| 109 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -3.roles in {BINARY}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {CALL}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.851. Support: 251.` |
| 110 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -3.roles not in {BINARY}<br>	∧ +1.reserved = )<br>	∧ +1.length ≤ 1<br>	∧ +2.length ≤ 1<br>	∧ ^1.roles not in {CALL}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.978. Support: 1683.` |
| 111 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -3.roles not in {BINARY}<br>	∧ +1.reserved not in {), =, {, }}<br>	∧ +1.length ≤ 1<br>	∧ +2.length ≤ 1<br>	∧ +3.reserved = ;<br>	∧ ^1.roles not in {CALL}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.995. Support: 293.` |
| 112 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -3.roles not in {BINARY}<br>	∧ -5.diff_line ≥ 1<br>	∧ +1.reserved not in {), =, {, }}<br>	∧ +1.length ≤ 1<br>	∧ +2.length ≤ 1<br>	∧ +3.reserved not in {;}<br>	∧ ^1.roles not in {CALL}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.887. Support: 128.` |
| 113 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, if}<br>	∧ +1.reserved = )<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.926. Support: 494.` |
| 114 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {,}<br>	∧ +1.reserved not in {), =, {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = '<br>Confidence: 0.868. Support: 178.` |
| 115 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {,, if}<br>	∧ -2.reserved = [<br>	∧ +1.reserved not in {), =, {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.938. Support: 184.` |
| 116 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {,, if}<br>	∧ +1.reserved not in {(, ), }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.934. Support: 4108.` |
| 117 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = }<br>	∧ -4.diff_col ≤ 12<br>	∧ +1.reserved = /<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved = ><br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.944. Support: 492.` |
| 118 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, if}<br>	∧ +1.reserved = /<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved = ><br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.994. Support: 1454.` |
| 119 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, if}<br>	∧ +1.reserved not in {/, =, {, }}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved = ><br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.885. Support: 370.` |
| 120 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.987. Support: 1556.` |
| 121 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = =<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {>}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = "<br>Confidence: 0.811. Support: 230.` |
| 122 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, =}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {>}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = '<br>Confidence: 0.901. Support: 1146.` |
| 123 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = return<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.887. Support: 934.` |
| 124 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, return}<br>	∧ -5.reserved = const<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.888. Support: 514.` |
| 125 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, if, return}<br>	∧ -5.reserved not in {const}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {>}<br>	∧ ^1.roles in {CONDITION} and not in {OPERATOR}<br>	∧ ^2.roles in {IF}<br>⇒ y = ∅<br>Confidence: 0.939. Support: 520.` |
| 126 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, return}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {INITIALIZATION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 168.` |
| 127 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = const<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 156.` |
| 128 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, if, return}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ -2.reserved = =<br>	∧ -3.roles in {INCOMPLETE}<br>	∧ -5.reserved not in {const}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {>}<br>	∧ ^1.roles not in {CONDITION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.979. Support: 312.` |
| 129 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, return}<br>	∧ -2.reserved = =<br>	∧ -3.roles not in {INCOMPLETE}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.929. Support: 274.` |
| 130 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = )<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = )<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {>}<br>	∧ ^1.roles not in {CONDITION, OPERATOR}<br>	∧ ^2.internal_type = FunctionExpression<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.974. Support: 559.` |
| 131 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {), ,, :, ;, const, if, return}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ -5.reserved not in {const}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = )<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {>}<br>	∧ ^1.roles not in {CONDITION, OPERATOR}<br>	∧ ^2.internal_type = FunctionExpression<br>⇒ y = ∅<br>Confidence: 0.911. Support: 974.` |
| 132 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, if, return}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ -2.reserved not in {=}<br>	∧ -5.reserved not in {const}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), =, {, }}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {>}<br>	∧ ^1.roles not in {CONDITION, OPERATOR}<br>	∧ ^2.internal_type = FunctionExpression<br>⇒ y = ∅<br>Confidence: 0.987. Support: 2271.` |
| 133 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {>}<br>	∧ +2.roles in {INCOMPLETE}<br>	∧ +3.reserved = ><br>	∧ ^1.roles in {CALL} and not in {CONDITION, OPERATOR}<br>	∧ ^2.internal_type not in {FunctionExpression}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.811. Support: 198.` |
| 134 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ -2.reserved not in {=}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {>}<br>	∧ +2.roles in {INCOMPLETE}<br>	∧ +3.length ≤ 6<br>	∧ +4.roles not in {IDENTIFIER}<br>	∧ ^1.roles in {CALL} and not in {CONDITION, OPERATOR}<br>	∧ ^2.internal_type not in {FunctionExpression}<br>⇒ y = ∅<br>Confidence: 0.887. Support: 748.` |
| 135 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {>}<br>	∧ +2.roles in {INCOMPLETE}<br>	∧ ^1.roles not in {CALL, CONDITION, OPERATOR}<br>	∧ ^2.internal_type not in {FunctionExpression}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.972. Support: 300.` |
| 136 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ,, :, ;, const, if, return}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ -2.reserved not in {=}<br>	∧ -5.reserved not in {const}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {>}<br>	∧ +2.roles in {INCOMPLETE}<br>	∧ ^1.roles not in {CONDITION, OPERATOR}<br>	∧ ^2.internal_type not in {FunctionExpression}<br>⇒ y = ∅<br>Confidence: 0.985. Support: 4235.` |
| 137 | `  -1.reserved not in {,, :, ;, const, if, return}<br>	∧ -2.internal_type = JSXIdentifier<br>	∧ -2.reserved not in {=}<br>	∧ -5.reserved not in {const}<br>	∧ +1.reserved not in {), =, {, }}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {>}<br>	∧ ^1.roles not in {CONDITION, OPERATOR}<br>	∧ ^2.internal_type not in {FunctionExpression}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 545.` |
| 138 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, if, return}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -2.internal_type not in {JSXIdentifier}<br>	∧ -2.reserved not in {=}<br>	∧ -5.reserved not in {const}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {>}<br>	∧ ^1.roles not in {CONDITION, OPERATOR}<br>	∧ ^2.internal_type not in {FunctionExpression}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 31948.` |
| 139 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, return}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label in {<newline>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = (<br>	∧ +1.length ≤ 1<br>	∧ +2.roles not in {INCOMPLETE}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {FunctionExpression}<br>⇒ y = ␣<br>Confidence: 0.939. Support: 351.` |
| 140 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, if, return}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.diff_col ≥ 7<br>	∧ -2.internal_type not in {JSXIdentifier}<br>	∧ -2.label not in {<newline>}<br>	∧ -5.reserved not in {const}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = (<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {>}<br>	∧ ^1.roles not in {CONDITION, OPERATOR}<br>	∧ ^2.internal_type not in {FunctionExpression}<br>⇒ y = ∅<br>Confidence: 0.947. Support: 1636.` |
| 141 | `  -1.diff_col ≥ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, return}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.diff_col ≤ 6<br>	∧ -2.label not in {<newline>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = (<br>	∧ +1.length ≤ 1<br>	∧ +2.roles not in {INCOMPLETE}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {FunctionExpression}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 327.` |
| 142 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, if, return}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.diff_col ≤ 6<br>	∧ -2.internal_type not in {JSXIdentifier}<br>	∧ -2.label not in {<newline>, <space>}<br>	∧ -5.reserved not in {const}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = (<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {>}<br>	∧ ^1.roles not in {CONDITION, OPERATOR}<br>	∧ ^2.internal_type not in {FunctionExpression}<br>⇒ y = ∅<br>Confidence: 0.979. Support: 504.` |
| 143 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = )<br>	∧ -5.diff_line ≥ 1<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = )<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {>}<br>	∧ +2.roles not in {INCOMPLETE}<br>	∧ ^1.roles not in {CONDITION, OPERATOR}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.925. Support: 181.` |
| 144 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = )<br>	∧ -2.reserved = }<br>	∧ -5.diff_line = 0<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = )<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {>}<br>	∧ +2.roles not in {INCOMPLETE}<br>	∧ ^1.roles not in {CONDITION, OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.855. Support: 141.` |
| 145 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = )<br>	∧ -2.internal_type not in {JSXIdentifier}<br>	∧ -5.diff_line = 0<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = )<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {>}<br>	∧ ^1.roles not in {CONDITION, OPERATOR}<br>	∧ ^2.internal_type not in {FunctionExpression}<br>⇒ y = ∅<br>Confidence: 0.846. Support: 1405.` |
| 146 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {), ,, :, ;, const, if, return}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.internal_type not in {JSXIdentifier}<br>	∧ -5.reserved not in {const}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = )<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {>}<br>	∧ ^1.roles not in {CONDITION, OPERATOR}<br>	∧ ^2.internal_type not in {FunctionExpression}<br>⇒ y = ∅<br>Confidence: 0.984. Support: 13367.` |
| 147 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, if, return}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.internal_type not in {JSXIdentifier}<br>	∧ -2.reserved not in {=}<br>	∧ -5.reserved not in {const}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), =, {, }}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {=, >}<br>	∧ ^1.roles not in {CONDITION, OPERATOR}<br>	∧ ^2.internal_type not in {FunctionExpression}<br>⇒ y = ∅<br>Confidence: 0.987. Support: 46941.` |
| 148 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, return}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length = 0<br>	∧ +2.reserved not in {>}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.984. Support: 469.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 10.75, "max_conf": 0.9999798536300659, "max_support": 46941, "min_conf": 0.80078125, "min_support": 128, "num_rules": 148}}
```
</details>
