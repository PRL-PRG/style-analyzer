# Model report for file:///tmp/top-repos-quality-repos-8_frcltu/node-open-mining-portal.git HEAD 8a673b6d7ce22d3ec1a3b5115378f0d4d72df581

### Dump

```json
{'created_at': '2021-08-29 21:36:49',
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
 'size': '17.2 kB',
 'tags': [],
 'uuid': '9021b50a-566a-4922-a564-52607d24415a',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-8_frcltu/node-open-mining-portal.git 8a673b6d7ce22d3ec1a3b5115378f0d4d72df581

# javascript
17 rules, avg.len. 5.2
## train
PPCR: 0.896663
### report
macro
{'f1-score': 0.5826547858166149,
 'precision': 0.5900817864013252,
 'recall': 0.5773657828194312,
 'support': 17007}
micro
{'f1-score': 0.9547245251955078,
 'precision': 0.9547245251955078,
 'recall': 0.9547245251955078,
 'support': 17007}
weighted
{'f1-score': 0.9515132027378447,
 'precision': 0.9492685181737226,
 'recall': 0.9547245251955078,
 'support': 17007}
### report_full
macro
{'f1-score': 0.5642728114586383,
 'precision': 0.5900817864013252,
 'recall': 0.5424266651210181,
 'support': 18967}
micro
{'f1-score': 0.9027075109801523,
 'precision': 0.9547245251955078,
 'recall': 0.8560657984921178,
 'support': 18967}
weighted
{'f1-score': 0.8736331798736843,
 'precision': 0.8928845543413,
 'recall': 0.8560657984921178,
 'support': 18967}
## test
PPCR: 0.867114
### report
macro
{'f1-score': 0.5802461624974213,
 'precision': 0.5850172073305033,
 'recall': 0.5764510043472623,
 'support': 4398}
micro
{'f1-score': 0.9404274670304684,
 'precision': 0.9404274670304684,
 'recall': 0.9404274670304684,
 'support': 4398}
weighted
{'f1-score': 0.9371046253984566,
 'precision': 0.9341927939289293,
 'recall': 0.9404274670304684,
 'support': 4398}
### report_full
macro
{'f1-score': 0.5599287064790746,
 'precision': 0.5850172073305033,
 'recall': 0.5383342306503286,
 'support': 5072}
micro
{'f1-score': 0.8734952481520591,
 'precision': 0.9404274670304684,
 'recall': 0.8154574132492114,
 'support': 5072}
weighted
{'f1-score': 0.842466564876034,
 'precision': 0.8726531720010197,
 'recall': 0.8154574132492114,
 'support': 5072}
```

## javascript
### Summary
11 rules, avg.len. 4.6

| | |
|-|-|
|Min support|102|
|Max support|5656|
|Min confidence|0.9465174078941345|
|Max confidence|0.99956214427948|

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
| 1 | `  -1.internal_type = StringLiteral<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = '<br>Confidence: 0.997. Support: 149.` |
| 2 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = '<br>Confidence: 0.995. Support: 102.` |
| 3 | `  +1.length ≥ 2<br>	∧ +2.reserved not in {]}<br>	∧ ^1.roles in {QUALIFIED} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 1142.` |
| 4 | `  -1.reserved = (<br>	∧ +1.roles in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = '<br>Confidence: 0.968. Support: 173.` |
| 5 | `  -1.reserved = (<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 839.` |
| 6 | `  -1.reserved = {<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.947. Support: 402.` |
| 7 | `  -1.label in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = '<br>Confidence: 0.987. Support: 195.` |
| 8 | `  -1.roles in {STRING}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = '<br>Confidence: 0.999. Support: 454.` |
| 9 | `  -1.roles not in {STRING}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 310.` |
| 10 | `  -1.reserved = if<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.981. Support: 183.` |
| 11 | `  -1.reserved not in {if}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.983. Support: 5656.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 4.636363636363637, "max_conf": 0.99956214427948, "max_support": 5656, "min_conf": 0.9465174078941345, "min_support": 102, "num_rules": 11}}
```
</details>
