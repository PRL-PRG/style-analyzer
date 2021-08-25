# Model report for file:///tmp/top-repos-quality-repos-aes85ail/campus-navigator.git HEAD 42004fbd9b618b68d7430d91d10190ecac7dc7d4

### Dump

```json
{'created_at': '2021-08-20 20:10:29',
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
 'size': '18.9 kB',
 'tags': [],
 'uuid': '941b7d5f-8220-4282-870d-283f063a2d0e',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-aes85ail/campus-navigator.git 42004fbd9b618b68d7430d91d10190ecac7dc7d4

# javascript
17 rules, avg.len. 6.6
## train
PPCR: 0.779812
### report
macro
{'f1-score': 0.518978586149316,
 'precision': 0.5449130123325812,
 'recall': 0.5030990822590319,
 'support': 11627}
micro
{'f1-score': 0.9304205728046787,
 'precision': 0.9304205728046787,
 'recall': 0.9304205728046787,
 'support': 11627}
weighted
{'f1-score': 0.9209545736752058,
 'precision': 0.9138310590967151,
 'recall': 0.9304205728046787,
 'support': 11627}
### report_full
macro
{'f1-score': 0.4266249521890069,
 'precision': 0.5449130123325812,
 'recall': 0.359178534868269,
 'support': 14910}
micro
{'f1-score': 0.8153144665938123,
 'precision': 0.9304205728046787,
 'recall': 0.7255533199195171,
 'support': 14910}
weighted
{'f1-score': 0.7898512961489993,
 'precision': 0.8796615852746765,
 'recall': 0.7255533199195171,
 'support': 14910}
## test
PPCR: 0.721649
### report
macro
{'f1-score': 0.41281681684556826,
 'precision': 0.44454353442256667,
 'recall': 0.4021541829548041,
 'support': 490}
micro
{'f1-score': 0.8551020408163266,
 'precision': 0.8551020408163266,
 'recall': 0.8551020408163266,
 'support': 490}
weighted
{'f1-score': 0.8339784900736137,
 'precision': 0.8220632664460841,
 'recall': 0.8551020408163266,
 'support': 490}
### report_full
macro
{'f1-score': 0.3335369854794214,
 'precision': 0.44454353442256667,
 'recall': 0.2865936566551911,
 'support': 679}
micro
{'f1-score': 0.716852010265184,
 'precision': 0.8551020408163266,
 'recall': 0.6170839469808542,
 'support': 679}
weighted
{'f1-score': 0.6768336395184357,
 'precision': 0.7785758683253813,
 'recall': 0.6170839469808542,
 'support': 679}
```

## javascript
### Summary
10 rules, avg.len. 7.0

| | |
|-|-|
|Min support|90|
|Max support|5496|
|Min confidence|0.9387736320495605|
|Max confidence|0.996874988079071|

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
| 1 | `  -1.label in {<space>}<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = '<br>Confidence: 0.957. Support: 127.` |
| 2 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {{}<br>	∧ +1.reserved = {<br>	∧ ^1.roles in {DECLARATION, FUNCTION}<br>⇒ y = ␣<br>Confidence: 0.995. Support: 94.` |
| 3 | `  -1.internal_type = CommentLine<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ⏎<br>Confidence: 0.980. Support: 380.` |
| 4 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.roles in {LITERAL}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 160.` |
| 5 | `  -1.diff_col ≥ 3<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.reserved = =<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.970. Support: 284.` |
| 6 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.982. Support: 192.` |
| 7 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = import<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.995. Support: 99.` |
| 8 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = {<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {INCOMPLETE} and not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.974. Support: 136.` |
| 9 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, import, {}<br>	∧ -3.reserved = ;<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {EXPRESSION} and not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.961. Support: 90.` |
| 10 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, import, {}<br>	∧ -3.reserved not in {;}<br>	∧ -5.reserved not in {;}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {DECLARATION, IF, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.939. Support: 5496.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 7.0, "max_conf": 0.996874988079071, "max_support": 5496, "min_conf": 0.9387736320495605, "min_support": 90, "num_rules": 10}}
```
</details>
