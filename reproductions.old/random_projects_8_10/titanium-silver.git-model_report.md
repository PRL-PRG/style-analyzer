# Model report for file:///tmp/top-repos-quality-repos-4ilvzqwh/titanium-silver.git HEAD 138aaad297e822640162deb1284509c07b312877

### Dump

```json
{'created_at': '2021-08-21 03:50:44',
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
 'size': '18.4 kB',
 'tags': [],
 'uuid': '292644e1-5f01-4009-9829-3d8c4f2bf7cc',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-4ilvzqwh/titanium-silver.git 138aaad297e822640162deb1284509c07b312877

# javascript
12 rules, avg.len. 6.3
## train
PPCR: 0.846141
### report
macro
{'f1-score': 0.33114721539651704,
 'precision': 0.3701202695310256,
 'recall': 0.3177552931221659,
 'support': 15239}
micro
{'f1-score': 0.9433689874663692,
 'precision': 0.9433689874663692,
 'recall': 0.9433689874663692,
 'support': 15239}
weighted
{'f1-score': 0.9288272254469234,
 'precision': 0.9208305718683829,
 'recall': 0.9433689874663692,
 'support': 15239}
### report_full
macro
{'f1-score': 0.2785824911022834,
 'precision': 0.3701202695310256,
 'recall': 0.2596956539684653,
 'support': 18010}
micro
{'f1-score': 0.8647478119642696,
 'precision': 0.9433689874663692,
 'recall': 0.798223209328151,
 'support': 18010}
weighted
{'f1-score': 0.8202682968277359,
 'precision': 0.8841644122062509,
 'recall': 0.798223209328151,
 'support': 18010}
## test
PPCR: 0.845101
### report
macro
{'f1-score': 0.29286530518564124,
 'precision': 0.3483063111401837,
 'recall': 0.28770198771581945,
 'support': 4261}
micro
{'f1-score': 0.9537667214268951,
 'precision': 0.9537667214268951,
 'recall': 0.9537667214268951,
 'support': 4261}
weighted
{'f1-score': 0.9442215027224024,
 'precision': 0.9393822872284815,
 'recall': 0.9537667214268951,
 'support': 4261}
### report_full
macro
{'f1-score': 0.26459075045411795,
 'precision': 0.3483063111401837,
 'recall': 0.2519125335743536,
 'support': 5042}
micro
{'f1-score': 0.873696656992368,
 'precision': 0.9537667214268951,
 'recall': 0.8060293534311781,
 'support': 5042}
weighted
{'f1-score': 0.8326333789092701,
 'precision': 0.8918331466974229,
 'recall': 0.8060293534311781,
 'support': 5042}
```

## javascript
### Summary
6 rules, avg.len. 6.0

| | |
|-|-|
|Min support|91|
|Max support|5905|
|Min confidence|0.9202473759651184|
|Max confidence|0.9814814925193787|

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
| 1 | `  -1.internal_type = StringLiteral<br>⇒ y = "<br>Confidence: 0.954. Support: 755.` |
| 2 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.970. Support: 5905.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {IDENTIFIER} and not in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.968. Support: 1440.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ -2.label in {<space>}<br>	∧ -3.internal_type = Identifier<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {IDENTIFIER, INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.981. Support: 189.` |
| 5 | `  -1.diff_col ≥ 4<br>	∧ -1.internal_type = CommentLine<br>	∧ -1.reserved not in {;}<br>	∧ -2.label not in {<space>}<br>	∧ +1.length ≥ 2<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {IDENTIFIER, INCOMPLETE}<br>⇒ y = ⏎<br>Confidence: 0.940. Support: 91.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.label not in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {IDENTIFIER, INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.920. Support: 1536.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 6.0, "max_conf": 0.9814814925193787, "max_support": 5905, "min_conf": 0.9202473759651184, "min_support": 91, "num_rules": 6}}
```
</details>
