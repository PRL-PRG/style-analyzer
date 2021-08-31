# Model report for file:///tmp/top-repos-quality-repos-_q_rk6px/doodlemaster.git HEAD 452cf529a1579dbbbe92d60b7fb5141a608f4a1b

### Dump

```json
{'created_at': '2021-08-31 11:26:12',
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
 'size': '17.0 kB',
 'tags': [],
 'uuid': '14ec8ffb-08e4-4845-9539-a295d6700522',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-_q_rk6px/doodlemaster.git 452cf529a1579dbbbe92d60b7fb5141a608f4a1b

# javascript
19 rules, avg.len. 6.8
## train
PPCR: 0.867798
### report
macro
{'f1-score': 0.8356518812146823,
 'precision': 0.8370772046169279,
 'recall': 0.8358424379401365,
 'support': 26539}
micro
{'f1-score': 0.9831568634839293,
 'precision': 0.9831568634839293,
 'recall': 0.9831568634839293,
 'support': 26539}
weighted
{'f1-score': 0.9818342698774007,
 'precision': 0.9807730553643442,
 'recall': 0.9831568634839293,
 'support': 26539}
### report_full
macro
{'f1-score': 0.6363977037251094,
 'precision': 0.8370772046169279,
 'recall': 0.5811794756382638,
 'support': 30582}
micro
{'f1-score': 0.9135694403109189,
 'precision': 0.9831568634839293,
 'recall': 0.853181610097443,
 'support': 30582}
weighted
{'f1-score': 0.8875640198055289,
 'precision': 0.9682082049141555,
 'recall': 0.853181610097443,
 'support': 30582}
## test
PPCR: 0.871542
### report
macro
{'f1-score': 0.38468323905689406,
 'precision': 0.41025320561240586,
 'recall': 0.4852063596628814,
 'support': 882}
micro
{'f1-score': 0.8276643990929705,
 'precision': 0.8276643990929705,
 'recall': 0.8276643990929705,
 'support': 882}
weighted
{'f1-score': 0.8117003610533949,
 'precision': 0.8227971012129298,
 'recall': 0.8276643990929705,
 'support': 882}
### report_full
macro
{'f1-score': 0.3518665629661842,
 'precision': 0.41025320561240586,
 'recall': 0.4373200721353533,
 'support': 1012}
micro
{'f1-score': 0.7708553326293558,
 'precision': 0.8276643990929705,
 'recall': 0.7213438735177866,
 'support': 1012}
weighted
{'f1-score': 0.7350425219772668,
 'precision': 0.7957109093447362,
 'recall': 0.7213438735177866,
 'support': 1012}
```

## javascript
### Summary
16 rules, avg.len. 6.9

| | |
|-|-|
|Min support|101|
|Max support|5505|
|Min confidence|0.951654314994812|
|Max confidence|0.9994663596153259|

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
| 1 | `  ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.988. Support: 5505.` |
| 2 | `  -1.label not in {<space>}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved = ]<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 101.` |
| 3 | `  •••start_col ≥ 37<br>	∧ -1.label not in {<space>}<br>	∧ -1.roles not in {STRING}<br>	∧ -4.diff_offset ≥ 7<br>	∧ +1.reserved not in {]}<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 351.` |
| 4 | `  •••start_col ≥ 37<br>	∧ -1.label not in {<space>}<br>	∧ -1.roles not in {STRING}<br>	∧ -4.diff_offset ≥ 7<br>	∧ +1.reserved not in {]}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +2.roles not in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.995. Support: 329.` |
| 5 | `  •••start_col ≤ 36<br>	∧ -1.label not in {<space>}<br>	∧ -1.roles not in {STRING}<br>	∧ -4.diff_offset ≥ 7<br>	∧ +1.reserved not in {]}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.993. Support: 1460.` |
| 6 | `  -1.reserved = {<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.992. Support: 785.` |
| 7 | `  -1.reserved not in {{}<br>	∧ +1.reserved = }<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.954. Support: 780.` |
| 8 | `  -1.reserved not in {{}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved = =<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 521.` |
| 9 | `  -1.reserved not in {{}<br>	∧ -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ +1.reserved not in {=, }}<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 3708.` |
| 10 | `  -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 1686.` |
| 11 | `  -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ;<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 937.` |
| 12 | `  -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {COMMENT}<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 383.` |
| 13 | `  -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.diff_col ≥ 103<br>	∧ -2.diff_offset ≥ 20<br>	∧ +1.reserved not in {}}<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 107.` |
| 14 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {;, }}<br>	∧ +1.roles in {KEY}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.985. Support: 227.` |
| 15 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 132.` |
| 16 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {), ;, }}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.952. Support: 3506.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 6.9375, "max_conf": 0.9994663596153259, "max_support": 5505, "min_conf": 0.951654314994812, "min_support": 101, "num_rules": 16}}
```
</details>
