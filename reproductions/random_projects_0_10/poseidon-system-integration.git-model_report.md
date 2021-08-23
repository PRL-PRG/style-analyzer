# Model report for file:///tmp/top-repos-quality-repos-qdcgs1nj/poseidon-system-integration.git HEAD a500d187f7c57e618c8a4d2551b0c941485c4a60

### Dump

```json
{'created_at': '2021-08-22 21:19:53',
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
 'size': '18.1 kB',
 'tags': [],
 'uuid': '590ff688-d0c7-44b5-b19e-83cf3b76deb6',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-qdcgs1nj/poseidon-system-integration.git a500d187f7c57e618c8a4d2551b0c941485c4a60

# javascript
14 rules, avg.len. 7.5
## train
PPCR: 0.776605
### report
macro
{'f1-score': 0.2770728849306109,
 'precision': 0.2757099745858864,
 'recall': 0.2787373093082414,
 'support': 15011}
micro
{'f1-score': 0.9590966624475384,
 'precision': 0.9590966624475384,
 'recall': 0.9590966624475384,
 'support': 15011}
weighted
{'f1-score': 0.9505790113764113,
 'precision': 0.9430914175958324,
 'recall': 0.9590966624475384,
 'support': 15011}
### report_full
macro
{'f1-score': 0.24986340541206564,
 'precision': 0.2757099745858864,
 'recall': 0.23389930224710778,
 'support': 19329}
micro
{'f1-score': 0.8384973791496797,
 'precision': 0.9590966624475384,
 'recall': 0.7448393605463294,
 'support': 19329}
weighted
{'f1-score': 0.763469393998636,
 'precision': 0.787433997887876,
 'recall': 0.7448393605463294,
 'support': 19329}
## test
PPCR: 0.766898
### report
macro
{'f1-score': 0.2861348260878582,
 'precision': 0.2839016672380138,
 'recall': 0.28850320570477234,
 'support': 3642}
micro
{'f1-score': 0.9651290499725426,
 'precision': 0.9651290499725426,
 'recall': 0.9651290499725426,
 'support': 3642}
weighted
{'f1-score': 0.9588283436389369,
 'precision': 0.9530330407621301,
 'recall': 0.9651290499725426,
 'support': 3642}
### report_full
macro
{'f1-score': 0.24153963660524697,
 'precision': 0.2839016672380138,
 'recall': 0.22206031339464732,
 'support': 4749}
micro
{'f1-score': 0.8378024073411988,
 'precision': 0.9651290499725426,
 'recall': 0.7401558222783744,
 'support': 4749}
weighted
{'f1-score': 0.7653377176792026,
 'precision': 0.8048811856904835,
 'recall': 0.7401558222783744,
 'support': 4749}
```

## javascript
### Summary
11 rules, avg.len. 5.7

| | |
|-|-|
|Min support|120|
|Max support|2354|
|Min confidence|0.9250344038009644|
|Max confidence|0.9986876845359802|

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
| 1 | `  -1.roles not in {LITERAL}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 2354.` |
| 2 | `  -1.label not in {<space>}<br>	∧ -1.roles not in {STRING}<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.969. Support: 828.` |
| 3 | `  -1.label not in {<space>}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ +2.roles not in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.925. Support: 727.` |
| 4 | `  -1.label not in {<space>}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles in {BINARY}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles not in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.954. Support: 120.` |
| 5 | `  -1.internal_type = Identifier<br>	∧ +1.reserved = =<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 266.` |
| 6 | `  -1.internal_type = Identifier<br>	∧ +1.reserved not in {=}<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.989. Support: 1959.` |
| 7 | `  -1.internal_type not in {Identifier}<br>	∧ -1.reserved = (<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 928.` |
| 8 | `  -1.internal_type not in {Identifier}<br>	∧ -1.reserved not in {(}<br>	∧ +1.reserved = ;<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.961. Support: 578.` |
| 9 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ +1.reserved = ,<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 381.` |
| 10 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ +1.reserved = )<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.986. Support: 314.` |
| 11 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.reserved not in {), ,, ;, }}<br>	∧ +1.roles in {COMMENT}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 236.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 5.7272727272727275, "max_conf": 0.9986876845359802, "max_support": 2354, "min_conf": 0.9250344038009644, "min_support": 120, "num_rules": 11}}
```
</details>
