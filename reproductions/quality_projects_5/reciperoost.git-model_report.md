# Model report for file:///tmp/top-repos-quality-repos-p4d2p68m/reciperoost.git HEAD dd397d9dad2548a66bc84a91c3d0d57d10cda5cb

### Dump

```json
{'created_at': '2021-08-30 09:06:19',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-5.4.0-74-generic-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.7 (default, Oct 22 2018, 11:32:17) [GCC 8.2.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '16.8 kB',
 'tags': [],
 'uuid': '13bfe39e-def2-441f-bc47-7ce9c986b3b5',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-p4d2p68m/reciperoost.git dd397d9dad2548a66bc84a91c3d0d57d10cda5cb

# javascript
17 rules, avg.len. 6.6
## train
PPCR: 0.762948
### report
macro
{'f1-score': 0.6861757835379807,
 'precision': 0.7040392265884943,
 'recall': 0.6732267983037789,
 'support': 7837}
micro
{'f1-score': 0.9476840627791246,
 'precision': 0.9476840627791246,
 'recall': 0.9476840627791246,
 'support': 7837}
weighted
{'f1-score': 0.9432195795611225,
 'precision': 0.9399986105805063,
 'recall': 0.9476840627791246,
 'support': 7837}
### report_full
macro
{'f1-score': 0.6134511214263065,
 'precision': 0.7040392265884943,
 'recall': 0.5564179495347169,
 'support': 10272}
micro
{'f1-score': 0.8202551217626594,
 'precision': 0.9476840627791246,
 'recall': 0.7230334890965732,
 'support': 10272}
weighted
{'f1-score': 0.7871544096632401,
 'precision': 0.8716619909883592,
 'recall': 0.7230334890965732,
 'support': 10272}
## test
PPCR: 0.689837
### report
macro
{'f1-score': 0.6865582380881383,
 'precision': 0.6976972357181696,
 'recall': 0.6797673236053052,
 'support': 1310}
micro
{'f1-score': 0.9198473282442748,
 'precision': 0.9198473282442748,
 'recall': 0.9198473282442748,
 'support': 1310}
weighted
{'f1-score': 0.9137606026720753,
 'precision': 0.9090744403680264,
 'recall': 0.9198473282442748,
 'support': 1310}
### report_full
macro
{'f1-score': 0.5947752342005155,
 'precision': 0.6976972357181696,
 'recall': 0.5349256492504043,
 'support': 1899}
micro
{'f1-score': 0.7510127765659084,
 'precision': 0.9198473282442748,
 'recall': 0.6345444971037388,
 'support': 1899}
weighted
{'f1-score': 0.7235872007156329,
 'precision': 0.8537171159849383,
 'recall': 0.6345444971037388,
 'support': 1899}
```

## javascript
### Summary
8 rules, avg.len. 5.6

| | |
|-|-|
|Min support|118|
|Max support|3493|
|Min confidence|0.9261904954910278|
|Max confidence|0.9984025359153748|

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
| 1 | `  -1.internal_type = StringLiteral<br>	∧ -3.internal_type = JSXIdentifier<br>⇒ y = "<br>Confidence: 0.996. Support: 141.` |
| 2 | `  -1.internal_type = StringLiteral<br>	∧ -3.internal_type not in {JSXIdentifier}<br>⇒ y = '<br>Confidence: 0.998. Support: 313.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^2.roles in {BLOCK}<br>⇒ y = "<br>Confidence: 0.996. Support: 133.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^2.roles not in {BLOCK}<br>⇒ y = '<br>Confidence: 0.998. Support: 226.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles not in {EXPRESSION}<br>	∧ ^2.roles not in {BLOCK}<br>⇒ y = ␣<br>Confidence: 0.942. Support: 232.` |
| 6 | `  -1.diff_offset ≥ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.reserved = =<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.926. Support: 210.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = )<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {STATEMENT} and not in {DECLARATION, OPERATOR}<br>	∧ ^2.roles not in {FILE}<br>⇒ y = ∅<br>Confidence: 0.945. Support: 118.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, }}<br>	∧ -3.diff_offset ≥ 4<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR, STATEMENT}<br>	∧ ^2.roles not in {FILE}<br>⇒ y = ∅<br>Confidence: 0.968. Support: 3493.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 5.625, "max_conf": 0.9984025359153748, "max_support": 3493, "min_conf": 0.9261904954910278, "min_support": 118, "num_rules": 8}}
```
</details>
