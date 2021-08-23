# Model report for file:///tmp/top-repos-quality-repos-iecphu_t/alaio-oracle.git HEAD dc4ba8da61304656df1eded68c814b3f8fe3beed

### Dump

```json
{'created_at': '2021-08-20 23:49:42',
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
 'size': '15.1 kB',
 'tags': [],
 'uuid': 'a2202564-dad0-4ab2-b779-fbeaa86a3c2c',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-iecphu_t/alaio-oracle.git dc4ba8da61304656df1eded68c814b3f8fe3beed

# javascript
2 rules, avg.len. 2.5
## train
PPCR: 0.183531
### report
macro
{'f1-score': 0.2798465070023845,
 'precision': 0.2773923173878642,
 'recall': 0.28234470258691713,
 'support': 711}
micro
{'f1-score': 0.9732770745428974,
 'precision': 0.9732770745428974,
 'recall': 0.9732770745428974,
 'support': 711}
weighted
{'f1-score': 0.9642958357700061,
 'precision': 0.9554791977410989,
 'recall': 0.9732770745428974,
 'support': 711}
### report_full
macro
{'f1-score': 0.09139457237527362,
 'precision': 0.2773923173878642,
 'recall': 0.05661453694998682,
 'support': 3874}
micro
{'f1-score': 0.3018538713195202,
 'precision': 0.9732770745428974,
 'recall': 0.17862674238513165,
 'support': 3874}
weighted
{'f1-score': 0.2836330603216589,
 'precision': 0.7727540159690554,
 'recall': 0.17862674238513165,
 'support': 3874}
## test
PPCR: 0.279033
### report
macro
{'f1-score': 0.27449508741003503,
 'precision': 0.28108608454180806,
 'recall': 0.26901669758812613,
 'support': 531}
micro
{'f1-score': 0.9717514124293786,
 'precision': 0.9717514124293786,
 'recall': 0.9717514124293786,
 'support': 531}
weighted
{'f1-score': 0.9658082491991832,
 'precision': 0.9613671584239363,
 'recall': 0.9717514124293786,
 'support': 531}
### report_full
macro
{'f1-score': 0.12100799220656107,
 'precision': 0.28108608454180806,
 'recall': 0.0811143885873844,
 'support': 1903}
micro
{'f1-score': 0.42399342645850446,
 'precision': 0.9717514124293786,
 'recall': 0.27115081450341566,
 'support': 1903}
weighted
{'f1-score': 0.39189159472075913,
 'precision': 0.7823693179690133,
 'recall': 0.27115081450341566,
 'support': 1903}
```

## javascript
### Summary
2 rules, avg.len. 2.5

| | |
|-|-|
|Min support|103|
|Max support|461|
|Min confidence|0.9563106894493103|
|Max confidence|0.9750542044639587|

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
                     'max_depth': 5,
                     'min_samples_leaf': 91,
                     'min_samples_split': 214,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.975. Support: 461.` |
| 2 | `  -1.roles not in {STRING}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.956. Support: 103.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 2.5, "max_conf": 0.9750542044639587, "max_support": 461, "min_conf": 0.9563106894493103, "min_support": 103, "num_rules": 2}}
```
</details>
