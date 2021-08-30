# Model report for file:///tmp/top-repos-quality-repos-tag1cwe1/vanta.git HEAD 2f5cfabfd4388a362b337a9b74783cb96de92b4f

### Dump

```json
{'created_at': '2021-08-29 14:50:00',
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
 'size': '16.8 kB',
 'tags': [],
 'uuid': '952a583f-1641-4dda-8365-9d5a54bfc36f',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-tag1cwe1/vanta.git 2f5cfabfd4388a362b337a9b74783cb96de92b4f

# javascript
17 rules, avg.len. 7.4
## train
PPCR: 0.860287
### report
macro
{'f1-score': 0.6511680781239754,
 'precision': 0.6863001163185032,
 'recall': 0.6406068020113963,
 'support': 16145}
micro
{'f1-score': 0.934964385258594,
 'precision': 0.934964385258594,
 'recall': 0.934964385258594,
 'support': 16145}
weighted
{'f1-score': 0.926852417884301,
 'precision': 0.9228641813051652,
 'recall': 0.934964385258594,
 'support': 16145}
### report_full
macro
{'f1-score': 0.5801127356935676,
 'precision': 0.6863001163185032,
 'recall': 0.5324316458937515,
 'support': 18767}
micro
{'f1-score': 0.8647456461961504,
 'precision': 0.934964385258594,
 'recall': 0.8043374007566473,
 'support': 18767}
weighted
{'f1-score': 0.8491974386800576,
 'precision': 0.9129646519956175,
 'recall': 0.8043374007566473,
 'support': 18767}
## test
PPCR: 0.873821
### report
macro
{'f1-score': 0.5958275242026989,
 'precision': 0.6795596177346226,
 'recall': 0.5844425770668595,
 'support': 3892}
micro
{'f1-score': 0.882065775950668,
 'precision': 0.882065775950668,
 'recall': 0.882065775950668,
 'support': 3892}
weighted
{'f1-score': 0.8739292737903946,
 'precision': 0.8723479251422505,
 'recall': 0.882065775950668,
 'support': 3892}
### report_full
macro
{'f1-score': 0.5439973850924489,
 'precision': 0.6795596177346226,
 'recall': 0.506893451568506,
 'support': 4454}
micro
{'f1-score': 0.8226695422957104,
 'precision': 0.882065775950668,
 'recall': 0.7707678491243826,
 'support': 4454}
weighted
{'f1-score': 0.8103543883317735,
 'precision': 0.8723447397335292,
 'recall': 0.7707678491243826,
 'support': 4454}
```

## javascript
### Summary
8 rules, avg.len. 5.1

| | |
|-|-|
|Min support|121|
|Max support|4009|
|Min confidence|0.9214876294136047|
|Max confidence|0.9985294342041016|

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
| 1 | `  ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.980. Support: 4009.` |
| 2 | `  +1.reserved = )<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.921. Support: 121.` |
| 3 | `  -3.roles in {EXPRESSION}<br>	∧ +1.reserved not in {)}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.959. Support: 1194.` |
| 4 | `  -1.roles in {EXPRESSION}<br>	∧ ^1.internal_type = VariableDeclarator<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 242.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.959. Support: 2252.` |
| 6 | `  -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type = CommentLine<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 340.` |
| 7 | `  -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {CommentLine}<br>	∧ +1.reserved = }<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.978. Support: 252.` |
| 8 | `  -1.reserved not in {(, {}<br>	∧ -1.roles not in {COMMENT, EXPRESSION}<br>	∧ -1.length ≥ 2<br>	∧ +1.internal_type not in {CommentLine}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles in {SCOPE} and not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.984. Support: 280.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 5.125, "max_conf": 0.9985294342041016, "max_support": 4009, "min_conf": 0.9214876294136047, "min_support": 121, "num_rules": 8}}
```
</details>
