# Model report for file:///tmp/top-repos-quality-repos-otox_8nd/hydra.git HEAD f8181b840b1ba664f40f5b38ee9c8dd1dce7f776

### Dump

```json
{'created_at': '2021-08-24 16:20:00',
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
 'size': '19.4 kB',
 'tags': [],
 'uuid': '17ced49d-e08e-47a8-9c19-d663833a872c',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-otox_8nd/hydra.git f8181b840b1ba664f40f5b38ee9c8dd1dce7f776

# javascript
24 rules, avg.len. 6.9
## train
PPCR: 0.794652
### report
macro
{'f1-score': 0.5421689805478621,
 'precision': 0.5551740395130494,
 'recall': 0.5310109668152706,
 'support': 37057}
micro
{'f1-score': 0.9644871414307689,
 'precision': 0.9644871414307689,
 'recall': 0.9644871414307689,
 'support': 37057}
weighted
{'f1-score': 0.9627087811200827,
 'precision': 0.961359664965739,
 'recall': 0.9644871414307689,
 'support': 37057}
### report_full
macro
{'f1-score': 0.3894467459693003,
 'precision': 0.5551740395130494,
 'recall': 0.32073174041057284,
 'support': 46633}
micro
{'f1-score': 0.854128330744414,
 'precision': 0.9644871414307689,
 'recall': 0.7664314970085562,
 'support': 46633}
weighted
{'f1-score': 0.8140512182597334,
 'precision': 0.9061163378601274,
 'recall': 0.7664314970085562,
 'support': 46633}
## test
PPCR: 0.785801
### report
macro
{'f1-score': 0.5203706384701375,
 'precision': 0.534496519146494,
 'recall': 0.5123644926898896,
 'support': 8456}
micro
{'f1-score': 0.9603831598864712,
 'precision': 0.9603831598864712,
 'recall': 0.9603831598864712,
 'support': 8456}
weighted
{'f1-score': 0.9581864051165182,
 'precision': 0.9566147438035273,
 'recall': 0.9603831598864712,
 'support': 8456}
### report_full
macro
{'f1-score': 0.33536843181385,
 'precision': 0.534496519146494,
 'recall': 0.2772376773154913,
 'support': 10761}
micro
{'f1-score': 0.8451891554352917,
 'precision': 0.9603831598864712,
 'recall': 0.7546696403679956,
 'support': 10761}
weighted
{'f1-score': 0.7997385842357703,
 'precision': 0.9003343312098439,
 'recall': 0.7546696403679956,
 'support': 10761}
```

## javascript
### Summary
12 rules, avg.len. 6.7

| | |
|-|-|
|Min support|102|
|Max support|8774|
|Min confidence|0.921875|
|Max confidence|0.9982969760894775|

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
                     'min_samples_split': 240,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.990. Support: 8500.` |
| 2 | `  -1.reserved = (<br>	∧ -2.roles in {CALLEE}<br>	∧ +1.roles in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.927. Support: 158.` |
| 3 | `  -1.reserved = (<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 1295.` |
| 4 | `  •••start_col ≤ 15<br>	∧ -1.reserved = ;<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎⏎<br>Confidence: 0.958. Support: 323.` |
| 5 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.roles in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {FILE} and not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.967. Support: 381.` |
| 6 | `  •••start_line ≥ 254<br>	∧ -1.label in {<newline>} and not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {FILE, IDENTIFIER}<br>⇒ y = "<br>Confidence: 0.998. Support: 229.` |
| 7 | `  -1.label not in {<newline>, <space>}<br>	∧ -1.reserved = }<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {SCOPE} and not in {FILE, IDENTIFIER}<br>⇒ y = ⏎⏎<br>Confidence: 0.922. Support: 160.` |
| 8 | `  -1.label not in {<newline>, <space>}<br>	∧ -1.reserved = +<br>	∧ -5.label in {<newline>}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {FILE, IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.956. Support: 149.` |
| 9 | `  -1.label not in {<newline>, <space>}<br>	∧ -1.reserved = +<br>	∧ -3.roles in {STRING}<br>	∧ -5.label not in {<newline>}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {FILE, IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.975. Support: 102.` |
| 10 | `  -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, +, ;, {, }}<br>	∧ -4.diff_offset ≥ 6<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {FILE, IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.952. Support: 4354.` |
| 11 | `  -1.roles not in {STRING}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 1468.` |
| 12 | `  -1.reserved not in {function, if}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {BinaryExpression}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.969. Support: 8774.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 6.666666666666667, "max_conf": 0.9982969760894775, "max_support": 8774, "min_conf": 0.921875, "min_support": 102, "num_rules": 12}}
```
</details>
