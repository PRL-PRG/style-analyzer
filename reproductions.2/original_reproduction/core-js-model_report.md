# Model report for file:///tmp/top-repos-quality-repos-fp6ay4ue/core-js HEAD 4a85fe5f9678296bc9ffd5cfc44b32d34b18e52f

### Dump

```json
{'created_at': '2021-08-13 18:50:28',
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
 'size': '18.6 kB',
 'tags': [],
 'uuid': '2364778d-889e-4773-bc86-73856e3b67d3',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-fp6ay4ue/core-js 4a85fe5f9678296bc9ffd5cfc44b32d34b18e52f

# javascript
80 rules, avg.len. 9.1
## train
PPCR: 0.991248
### report
macro
{'f1-score': 0.8131623758836029,
 'precision': 0.8433412166190385,
 'recall': 0.7928075805966366,
 'support': 263218}
micro
{'f1-score': 0.9792187464383135,
 'precision': 0.9792187464383135,
 'recall': 0.9792187464383135,
 'support': 263218}
weighted
{'f1-score': 0.9785726421325892,
 'precision': 0.9785709192419897,
 'recall': 0.9792187464383135,
 'support': 263218}
### report_full
macro
{'f1-score': 0.8047200491979036,
 'precision': 0.8433412166190385,
 'recall': 0.7783286989205231,
 'support': 265542}
micro
{'f1-score': 0.9749148952265678,
 'precision': 0.9792187464383135,
 'recall': 0.9706487109383827,
 'support': 265542}
weighted
{'f1-score': 0.9740902733158994,
 'precision': 0.9784595728529899,
 'recall': 0.9706487109383827,
 'support': 265542}
## test
PPCR: 0.993106
### report
macro
{'f1-score': 0.8164025187535088,
 'precision': 0.8396623652196054,
 'recall': 0.8008224915891367,
 'support': 65836}
micro
{'f1-score': 0.9820766753751746,
 'precision': 0.9820766753751746,
 'recall': 0.9820766753751746,
 'support': 65836}
weighted
{'f1-score': 0.9814024144550657,
 'precision': 0.9813813644087964,
 'recall': 0.9820766753751746,
 'support': 65836}
### report_full
macro
{'f1-score': 0.8076444514789083,
 'precision': 0.8396623652196054,
 'recall': 0.7866676336238437,
 'support': 66293}
micro
{'f1-score': 0.9786799264355289,
 'precision': 0.9820766753751746,
 'recall': 0.9753065934563226,
 'support': 66293}
weighted
{'f1-score': 0.977759489445069,
 'precision': 0.9812003198347061,
 'recall': 0.9753065934563226,
 'support': 66293}
```

## javascript
### Summary
80 rules, avg.len. 9.1

| | |
|-|-|
|Min support|98|
|Max support|20029|
|Min confidence|0.8044217824935913|
|Max confidence|0.9997365474700928|

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
| 1 | `  -1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 0.996. Support: 12578.` |
| 2 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.roles in {MAP}<br>⇒ y = ⏎<br>Confidence: 0.972. Support: 488.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -5.label not in {<newline>}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.roles not in {MAP}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 2662.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = ␣<br>Confidence: 1.000. Support: 1898.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = [<br>	∧ -2.label in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 0.871. Support: 143.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, [}<br>	∧ -2.label in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = ␣<br>Confidence: 0.962. Support: 1547.` |
| 7 | `  -1.diff_offset ≤ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :}<br>	∧ -1.length ≥ 6<br>	∧ -2.label not in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = ␣<br>Confidence: 0.999. Support: 421.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :}<br>	∧ -1.length ≤ 5<br>	∧ -2.label not in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 0.994. Support: 12350.` |
| 9 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.974. Support: 2113.` |
| 10 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +3.reserved = exports<br>⇒ y = ⏎⏎<br>Confidence: 0.985. Support: 292.` |
| 11 | `  •••start_line ≤ 19<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +2.roles in {COMMENT}<br>	∧ +3.reserved not in {exports}<br>⇒ y = ⏎⏎<br>Confidence: 0.880. Support: 196.` |
| 12 | `  •••start_line ≥ 5<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ -3.diff_col ≥ 22<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +2.roles not in {COMMENT}<br>⇒ y = ⏎<br>Confidence: 0.855. Support: 472.` |
| 13 | `  •••start_line ≤ 4<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ -3.diff_col ≥ 22<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +2.roles not in {COMMENT}<br>⇒ y = ⏎⏎<br>Confidence: 0.804. Support: 294.` |
| 14 | `  •••start_col ≥ 11<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ -3.diff_col ≤ 21<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^2.internal_type not in {BlockStatement}<br>⇒ y = ⏎<br>Confidence: 0.953. Support: 10351.` |
| 15 | `  •••start_col ≤ 10<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ -3.diff_col ≤ 21<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.roles in {MODULE}<br>	∧ ^2.internal_type not in {BlockStatement}<br>⇒ y = ⏎⏎<br>Confidence: 0.970. Support: 183.` |
| 16 | `  •••start_col ≤ 10<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ -3.diff_col ≤ 21<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.roles not in {MODULE}<br>	∧ ^2.internal_type not in {BlockStatement}<br>⇒ y = ⏎<br>Confidence: 0.909. Support: 731.` |
| 17 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ ^1.roles in {MAP}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.999. Support: 528.` |
| 18 | `  •••start_line ≥ 20<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -2.roles not in {IDENTIFIER}<br>	∧ -3.roles in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles in {MAP}<br>⇒ y = ⏎<br>Confidence: 0.989. Support: 1222.` |
| 19 | `  •••start_col ≤ 27<br>	∧ •••start_line ≥ 20<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -2.roles not in {IDENTIFIER}<br>	∧ -3.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles in {MAP}<br>⇒ y = ⏎<br>Confidence: 0.929. Support: 460.` |
| 20 | `  •••start_col ≥ 34<br>	∧ •••start_line ≤ 19<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles in {MAP}<br>⇒ y = ␣<br>Confidence: 0.949. Support: 303.` |
| 21 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ ^1.roles not in {MAP}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.998. Support: 277.` |
| 22 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -3.reserved = ]<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {MAP}<br>	∧ ^2.roles in {VARIABLE}<br>⇒ y = ⏎<br>Confidence: 0.994. Support: 265.` |
| 23 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -3.reserved not in {]}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +2.roles in {COMMENT}<br>	∧ ^1.roles not in {MAP}<br>⇒ y = ⏎<br>Confidence: 0.958. Support: 180.` |
| 24 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -3.reserved not in {]}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {MAP}<br>⇒ y = ␣<br>Confidence: 0.982. Support: 10001.` |
| 25 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = )<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 856.` |
| 26 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ]<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 309.` |
| 27 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ]}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.991. Support: 268.` |
| 28 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ,, ;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = (<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.913. Support: 282.` |
| 29 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ,, ;}<br>	∧ -2.diff_col ≥ 3<br>	∧ -3.reserved not in {===}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {(, ), ]}<br>	∧ ^1.internal_type = LogicalExpression<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.943. Support: 1050.` |
| 30 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ,, ;}<br>	∧ -3.reserved not in {===}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {(, ), ]}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.985. Support: 9068.` |
| 31 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {VALUE}<br>	∧ ^1.roles in {DECLARATION} and not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 147.` |
| 32 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {VALUE}<br>	∧ ^1.roles in {DECLARATION} and not in {OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.967. Support: 1224.` |
| 33 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.reserved = {<br>	∧ ^1.roles in {DECLARATION} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.957. Support: 1185.` |
| 34 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ,<br>	∧ +2.reserved not in {{}<br>	∧ ^1.roles in {DECLARATION} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 723.` |
| 35 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {DECLARATION} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 411.` |
| 36 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ,, ;, {}<br>	∧ -3.reserved = function<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {,}<br>	∧ +2.reserved not in {{}<br>	∧ ^1.roles in {DECLARATION} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 285.` |
| 37 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -3.reserved not in {function}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ><br>	∧ ^1.roles in {DECLARATION} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 159.` |
| 38 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ,, ;, {}<br>	∧ -3.reserved not in {function}<br>	∧ -4.diff_offset ≥ 5<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {,, >}<br>	∧ +2.reserved not in {), {}<br>	∧ ^1.roles in {DECLARATION} and not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.990. Support: 8869.` |
| 39 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = =<br>	∧ -3.reserved not in {function}<br>	∧ -4.diff_offset ≤ 4<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {>}<br>	∧ +2.reserved not in {)}<br>	∧ ^1.roles in {DECLARATION} and not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 355.` |
| 40 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ,, ;, =, {}<br>	∧ -3.reserved not in {function}<br>	∧ -4.diff_offset ≤ 4<br>	∧ -4.label not in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {,}<br>	∧ +2.reserved not in {), {}<br>	∧ ^1.roles in {DECLARATION} and not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.858. Support: 378.` |
| 41 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +4.reserved = }<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.946. Support: 530.` |
| 42 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -3.diff_col ≥ 7<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {KEY}<br>	∧ +4.reserved not in {}}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.991. Support: 172.` |
| 43 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -2.reserved = (<br>	∧ -3.diff_col ≤ 6<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +4.reserved not in {}}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.876. Support: 149.` |
| 44 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -2.reserved not in {(}<br>	∧ -3.diff_col ≤ 6<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +4.reserved not in {}}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.956. Support: 2090.` |
| 45 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.992. Support: 2379.` |
| 46 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = var<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ␣<br>Confidence: 1.000. Support: 1819.` |
| 47 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 281.` |
| 48 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ,, ;, var, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.989. Support: 1856.` |
| 49 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, var}<br>	∧ -5.diff_line ≥ 1<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.931. Support: 372.` |
| 50 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, var, {}<br>	∧ -5.diff_line = 0<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.925. Support: 1098.` |
| 51 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {,, :, ;, var, {}<br>	∧ +1.reserved not in {{, }}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.972. Support: 942.` |
| 52 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 895.` |
| 53 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = if<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 781.` |
| 54 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = function<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 724.` |
| 55 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, function, var}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = )<br>	∧ ^1.internal_type = ConditionalExpression<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.988. Support: 121.` |
| 56 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), }}<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.internal_type = ConditionalExpression<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.939. Support: 619.` |
| 57 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = ?<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), }}<br>	∧ +2.roles not in {EXPRESSION}<br>	∧ ^1.internal_type = ConditionalExpression<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 255.` |
| 58 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = return<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 664.` |
| 59 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = const<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 599.` |
| 60 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ -2.reserved = =<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 529.` |
| 61 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = typeof<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 492.` |
| 62 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = new<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 457.` |
| 63 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = ${<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 307.` |
| 64 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ -3.diff_offset ≥ 5<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>	∧ ^2.roles in {BLOCK}<br>⇒ y = ␣<br>Confidence: 0.962. Support: 689.` |
| 65 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, typeof, var, {}<br>	∧ -2.label in {<space>}<br>	∧ -3.diff_offset ≤ 4<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>	∧ ^2.roles in {BLOCK}<br>⇒ y = ∅<br>Confidence: 0.902. Support: 107.` |
| 66 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ -2.label not in {<space>}<br>	∧ -3.diff_offset ≤ 4<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>	∧ ^2.roles in {BLOCK}<br>⇒ y = ␣<br>Confidence: 0.883. Support: 98.` |
| 67 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, typeof, var, {}<br>	∧ -5.diff_line = 0<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>	∧ ^2.roles in {BLOCK}<br>⇒ y = ∅<br>Confidence: 0.979. Support: 1384.` |
| 68 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = import<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 146.` |
| 69 | `  -1.internal_type = DirectiveLiteral<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>	∧ ^2.roles not in {BLOCK}<br>⇒ y = '<br>Confidence: 0.996. Support: 131.` |
| 70 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ +1.internal_type = DirectiveLiteral<br>	∧ +1.reserved not in {=, {, }}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>	∧ ^2.roles not in {BLOCK}<br>⇒ y = '<br>Confidence: 0.989. Support: 132.` |
| 71 | `  -1.internal_type not in {CommentLine, DirectiveLiteral, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, var, {}<br>	∧ -2.label in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>	∧ ^2.roles not in {BLOCK}<br>⇒ y = ∅<br>Confidence: 0.987. Support: 2499.` |
| 72 | `  -1.diff_offset ≥ 2<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ -2.label not in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +2.reserved = =<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.926. Support: 195.` |
| 73 | `  -1.diff_offset ≤ 1<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, return, var, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +2.reserved = =<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>	∧ ^2.roles not in {BLOCK}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 2137.` |
| 74 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, return, var, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.internal_type = IfStatement<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>	∧ ^2.roles not in {BLOCK}<br>⇒ y = ∅<br>Confidence: 0.836. Support: 198.` |
| 75 | `  -1.internal_type not in {CommentLine, DirectiveLiteral, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, var}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = function<br>	∧ ^1.internal_type not in {IfStatement}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.858. Support: 165.` |
| 76 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ -2.label in {<newline>} and not in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles in {STATEMENT} and not in {DECLARATION, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.810. Support: 182.` |
| 77 | `  -1.internal_type not in {CommentLine, DirectiveLiteral, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, var, {}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -2.label in {<newline>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.internal_type not in {IfStatement}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR, STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 10743.` |
| 78 | `  -1.internal_type not in {CommentLine, DirectiveLiteral, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, var, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label in {<newline>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles in {COMMENT}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.internal_type not in {IfStatement}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR, STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 948.` |
| 79 | `  -1.internal_type not in {CommentLine, DirectiveLiteral, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, var, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label in {<newline>}<br>	∧ -5.diff_offset ≤ 10<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.internal_type not in {IfStatement}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR, STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.972. Support: 340.` |
| 80 | `  -1.internal_type not in {CommentLine, DirectiveLiteral, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, var, {}<br>	∧ -2.label not in {<newline>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.internal_type not in {IfStatement}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 20029.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 9.075, "max_conf": 0.9997365474700928, "max_support": 20029, "min_conf": 0.8044217824935913, "min_support": 98, "num_rules": 80}}
```
</details>
