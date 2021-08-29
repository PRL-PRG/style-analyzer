# Model report for file:///tmp/top-repos-quality-repos-pced0zst/otus_web.git HEAD b90ad69e1b5c1828fa2ace165710422d113d1d17

### Dump

```json
{'created_at': '2021-08-29 02:30:20',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-4.19.0-12-amd64-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.7 (default, Oct 22 2018, 11:32:17) [GCC 8.2.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '20.4 kB',
 'tags': [],
 'uuid': 'c6624c24-cdbd-471f-85d1-51494bc434de',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-pced0zst/otus_web.git b90ad69e1b5c1828fa2ace165710422d113d1d17

# javascript
25 rules, avg.len. 7.8
## train
PPCR: 0.752934
### report
macro
{'f1-score': 0.36330972573810355,
 'precision': 0.3773285574399076,
 'recall': 0.3522736651725069,
 'support': 21878}
micro
{'f1-score': 0.923896151384953,
 'precision': 0.923896151384953,
 'recall': 0.923896151384953,
 'support': 21878}
weighted
{'f1-score': 0.9151752538182863,
 'precision': 0.9101288155117219,
 'recall': 0.923896151384953,
 'support': 21878}
### report_full
macro
{'f1-score': 0.27207300643783894,
 'precision': 0.3773285574399076,
 'recall': 0.2282878102505902,
 'support': 29057}
micro
{'f1-score': 0.7936782173358202,
 'precision': 0.923896151384953,
 'recall': 0.6956327218914548,
 'support': 29057}
weighted
{'f1-score': 0.7532006340315265,
 'precision': 0.8635501929424868,
 'recall': 0.6956327218914548,
 'support': 29057}
## test
PPCR: 0.762238
### report
macro
{'f1-score': 0.31276697992465263,
 'precision': 0.39903406536954983,
 'recall': 0.2978937679735552,
 'support': 872}
micro
{'f1-score': 0.930045871559633,
 'precision': 0.930045871559633,
 'recall': 0.930045871559633,
 'support': 872}
weighted
{'f1-score': 0.9158840131596229,
 'precision': 0.9163361450393148,
 'recall': 0.930045871559633,
 'support': 872}
### report_full
macro
{'f1-score': 0.23846892755916713,
 'precision': 0.39903406536954983,
 'recall': 0.20233194051240444,
 'support': 1144}
micro
{'f1-score': 0.8045634920634922,
 'precision': 0.930045871559633,
 'recall': 0.708916083916084,
 'support': 1144}
weighted
{'f1-score': 0.7553192157325753,
 'precision': 0.8812958364598222,
 'recall': 0.708916083916084,
 'support': 1144}
```

## javascript
### Summary
13 rules, avg.len. 6.5

| | |
|-|-|
|Min support|130|
|Max support|4112|
|Min confidence|0.9260985255241394|
|Max confidence|0.9961538314819336|

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
                     'min_samples_leaf': 120,
                     'min_samples_split': 238,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.933. Support: 4112.` |
| 2 | `  -1.internal_type = StringLiteral<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 0.946. Support: 809.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 0.976. Support: 446.` |
| 4 | `  -1.diff_offset ≥ 3<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles not in {COMMENT}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.972. Support: 1072.` |
| 5 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -3.roles not in {EXPRESSION}<br>	∧ -5.diff_line ≥ 1<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎<br>Confidence: 0.946. Support: 140.` |
| 6 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ -3.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 10<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.956. Support: 700.` |
| 7 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, {, }}<br>	∧ -3.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≤ 9<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {MAP}<br>⇒ y = ␣<br>Confidence: 0.973. Support: 165.` |
| 8 | `  •••start_line ≤ 168<br>	∧ -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ,, ;, {, }}<br>	∧ -3.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≤ 9<br>	∧ ^1.internal_type = JSXElement<br>	∧ ^1.roles not in {MAP}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 130.` |
| 9 | `  -1.internal_type not in {StringLiteral}<br>	∧ -3.diff_offset ≥ 7<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.966. Support: 601.` |
| 10 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.932. Support: 139.` |
| 11 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.926. Support: 751.` |
| 12 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = if<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.985. Support: 236.` |
| 13 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = function<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.930. Support: 164.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 6.538461538461538, "max_conf": 0.9961538314819336, "max_support": 4112, "min_conf": 0.9260985255241394, "min_support": 130, "num_rules": 13}}
```
</details>
