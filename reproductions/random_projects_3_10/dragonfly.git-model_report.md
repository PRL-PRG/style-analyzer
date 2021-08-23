# Model report for file:///tmp/top-repos-quality-repos-2y1_wsr9/dragonfly.git HEAD 891ac39a1ce9f879e56a5ad90964aaba6a7d5564

### Dump

```json
{'created_at': '2021-08-22 01:57:21',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-5.4.0-81-generic-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.7 (default, Oct 22 2018, 11:32:17) [GCC 8.2.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '17.1 kB',
 'tags': [],
 'uuid': 'bb0daad9-9221-492b-97db-9369f23b1632',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-2y1_wsr9/dragonfly.git 891ac39a1ce9f879e56a5ad90964aaba6a7d5564

# javascript
28 rules, avg.len. 7.0
## train
PPCR: 0.910809
### report
macro
{'f1-score': 0.7607584549098976,
 'precision': 0.8268940848414694,
 'recall': 0.7296677853290465,
 'support': 20148}
micro
{'f1-score': 0.9388028588445503,
 'precision': 0.9388028588445503,
 'recall': 0.9388028588445503,
 'support': 20148}
weighted
{'f1-score': 0.9348541950202768,
 'precision': 0.9363839208693743,
 'recall': 0.9388028588445503,
 'support': 20148}
### report_full
macro
{'f1-score': 0.6955073114955617,
 'precision': 0.8268940848414694,
 'recall': 0.6473148015459151,
 'support': 22121}
micro
{'f1-score': 0.8949821382100357,
 'precision': 0.9388028588445503,
 'recall': 0.8550698431354821,
 'support': 22121}
weighted
{'f1-score': 0.8754248367493301,
 'precision': 0.9197742241374067,
 'recall': 0.8550698431354821,
 'support': 22121}
## test
PPCR: 0.913557
### report
macro
{'f1-score': 0.7029756927482451,
 'precision': 0.7702904852245281,
 'recall': 0.6833469630282194,
 'support': 4407}
micro
{'f1-score': 0.9040163376446563,
 'precision': 0.9040163376446563,
 'recall': 0.9040163376446563,
 'support': 4407}
weighted
{'f1-score': 0.8990514741242683,
 'precision': 0.902143287930013,
 'recall': 0.9040163376446563,
 'support': 4407}
### report_full
macro
{'f1-score': 0.6501567046009331,
 'precision': 0.7702904852245281,
 'recall': 0.6216396515443836,
 'support': 4824}
micro
{'f1-score': 0.8631784205394867,
 'precision': 0.9040163376446563,
 'recall': 0.8258706467661692,
 'support': 4824}
weighted
{'f1-score': 0.8433078144371605,
 'precision': 0.8881520989437394,
 'recall': 0.8258706467661692,
 'support': 4824}
```

## javascript
### Summary
21 rules, avg.len. 6.8

| | |
|-|-|
|Min support|99|
|Max support|8504|
|Min confidence|0.934587836265564|
|Max confidence|0.9993780851364136|

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
| 1 | `  -1.internal_type = StringLiteral<br>	∧ -3.internal_type = JSXIdentifier<br>⇒ y = "<br>Confidence: 0.996. Support: 139.` |
| 2 | `  -1.internal_type = StringLiteral<br>	∧ -3.internal_type not in {JSXIdentifier}<br>⇒ y = '<br>Confidence: 0.999. Support: 804.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 0.999. Support: 521.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 0.998. Support: 255.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -2.roles in {INCOMPLETE}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = "<br>Confidence: 0.996. Support: 140.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -2.roles not in {INCOMPLETE}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = ␣<br>Confidence: 0.935. Support: 558.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.998. Support: 206.` |
| 8 | `  •••start_col ≥ 20<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = import<br>⇒ y = ⏎<br>Confidence: 0.996. Support: 113.` |
| 9 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ><br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 156.` |
| 10 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -3.reserved = ><br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {>}<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.955. Support: 99.` |
| 11 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ,<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 103.` |
| 12 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.941. Support: 211.` |
| 13 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ -2.roles in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.968. Support: 425.` |
| 14 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {INCOMPLETE} and not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.948. Support: 182.` |
| 15 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.reserved = =<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.992. Support: 193.` |
| 16 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = import<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 144.` |
| 17 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, import, {}<br>	∧ -2.reserved = =<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.970. Support: 118.` |
| 18 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, import, {}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.reserved = ><br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.963. Support: 147.` |
| 19 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, import, {}<br>	∧ -2.reserved not in {=}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +2.reserved not in {=, >}<br>	∧ ^1.roles in {INCOMPLETE} and not in {DECLARATION, OPERATOR}<br>	∧ ^2.internal_type not in {BlockStatement}<br>⇒ y = ∅<br>Confidence: 0.984. Support: 152.` |
| 20 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, import, {}<br>	∧ -2.label in {<newline>}<br>	∧ -2.reserved not in {=}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +2.reserved not in {=, >}<br>	∧ ^1.roles in {EXPRESSION} and not in {DECLARATION, OPERATOR}<br>	∧ ^2.internal_type not in {BlockStatement}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 133.` |
| 21 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, import, {}<br>	∧ -2.label not in {<newline>}<br>	∧ -2.reserved not in {=}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +2.reserved not in {=, >}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>	∧ ^2.internal_type not in {BlockStatement}<br>⇒ y = ∅<br>Confidence: 0.946. Support: 8504.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 6.761904761904762, "max_conf": 0.9993780851364136, "max_support": 8504, "min_conf": 0.934587836265564, "min_support": 99, "num_rules": 21}}
```
</details>
