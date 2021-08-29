# Model report for file:///tmp/top-repos-quality-repos-p3b_lq1g/coronavirus-tracker-cli.git HEAD 89c688e2cbcea6f16f10973030bca2262cd64d10

### Dump

```json
{'created_at': '2021-08-29 16:04:00',
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
 'size': '14.6 kB',
 'tags': [],
 'uuid': 'f5ffd4af-46e5-4c37-bfa3-f5a06c5b0853',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-p3b_lq1g/coronavirus-tracker-cli.git 89c688e2cbcea6f16f10973030bca2262cd64d10

# javascript
6 rules, avg.len. 4.8
## train
PPCR: 0.423617
### report
macro
{'f1-score': 0.31346544005218197,
 'precision': 0.3044388576266719,
 'recall': 0.323259034110098,
 'support': 1417}
micro
{'f1-score': 0.9124911785462244,
 'precision': 0.9124911785462244,
 'recall': 0.9124911785462244,
 'support': 1417}
weighted
{'f1-score': 0.8846369285157603,
 'precision': 0.8589716537252463,
 'recall': 0.9124911785462244,
 'support': 1417}
### report_full
macro
{'f1-score': 0.21567894685220468,
 'precision': 0.3044388576266719,
 'recall': 0.17516681076995613,
 'support': 3345}
micro
{'f1-score': 0.5430491390172196,
 'precision': 0.9124911785462244,
 'recall': 0.3865470852017937,
 'support': 3345}
weighted
{'f1-score': 0.49607734181735813,
 'precision': 0.7560403278151894,
 'recall': 0.3865470852017937,
 'support': 3345}
## test
PPCR: 0.444822
### report
macro
{'f1-score': 0.30635216359482775,
 'precision': 0.29209673249408347,
 'recall': 0.32608695652173914,
 'support': 262}
micro
{'f1-score': 0.8587786259541985,
 'precision': 0.8587786259541985,
 'recall': 0.8587786259541985,
 'support': 262}
weighted
{'f1-score': 0.8068053163375236,
 'precision': 0.769262387102739,
 'recall': 0.8587786259541985,
 'support': 262}
### report_full
macro
{'f1-score': 0.20935960591133004,
 'precision': 0.29209673249408347,
 'recall': 0.17423266263587403,
 'support': 589}
micro
{'f1-score': 0.5287896592244419,
 'precision': 0.8587786259541985,
 'recall': 0.38200339558573854,
 'support': 589}
weighted
{'f1-score': 0.4787692256224544,
 'precision': 0.7174433453681028,
 'recall': 0.38200339558573854,
 'support': 589}
```

## javascript
### Summary
3 rules, avg.len. 3.3

| | |
|-|-|
|Min support|132|
|Max support|545|
|Min confidence|0.9238532185554504|
|Max confidence|0.9962121248245239|

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
| 1 | `  ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.924. Support: 545.` |
| 2 | `  -1.diff_offset ≥ 3<br>	∧ -1.reserved not in {(}<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ +2.roles not in {IDENTIFIER}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.989. Support: 134.` |
| 3 | `  +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.996. Support: 132.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 3.3333333333333335, "max_conf": 0.9962121248245239, "max_support": 545, "min_conf": 0.9238532185554504, "min_support": 132, "num_rules": 3}}
```
</details>
