# Model report for file:///tmp/top-repos-quality-repos-0hb9ysqw/koa-graphql.git HEAD 7a4ed0578c4511baf210d44c542e0954c8e655e3

### Dump

```json
{'created_at': '2021-08-29 16:34:54',
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
 'size': '12.7 kB',
 'tags': [],
 'uuid': 'daec55d2-9258-4a33-ac2a-abf59113a8a9',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-0hb9ysqw/koa-graphql.git 7a4ed0578c4511baf210d44c542e0954c8e655e3

# javascript
3 rules, avg.len. 2.7
## train
PPCR: 0.457340
### report
macro
{'f1-score': 0.4469607116382506,
 'precision': 0.46957671957671954,
 'recall': 0.43577981651376146,
 'support': 729}
micro
{'f1-score': 0.9053497942386831,
 'precision': 0.9053497942386831,
 'recall': 0.9053497942386831,
 'support': 729}
weighted
{'f1-score': 0.8938393627957913,
 'precision': 0.8990354257843969,
 'recall': 0.9053497942386831,
 'support': 729}
### report_full
macro
{'f1-score': 0.2960546272656033,
 'precision': 0.46957671957671954,
 'recall': 0.2247689591914437,
 'support': 1594}
micro
{'f1-score': 0.56823073611709,
 'precision': 0.9053497942386831,
 'recall': 0.41405269761606023,
 'support': 1594}
weighted
{'f1-score': 0.5310330347539591,
 'precision': 0.7909256271866059,
 'recall': 0.41405269761606023,
 'support': 1594}
## test
PPCR: 0.300000
### report
macro
{'f1-score': 0.41428571428571426,
 'precision': 0.4375,
 'recall': 0.41666666666666663,
 'support': 6}
micro
{'f1-score': 0.8333333333333334,
 'precision': 0.8333333333333334,
 'recall': 0.8333333333333334,
 'support': 6}
weighted
{'f1-score': 0.8285714285714286,
 'precision': 0.875,
 'recall': 0.8333333333333334,
 'support': 6}
### report_full
macro
{'f1-score': 0.28205128205128205,
 'precision': 0.4375,
 'recall': 0.20833333333333331,
 'support': 20}
micro
{'f1-score': 0.3846153846153846,
 'precision': 0.8333333333333334,
 'recall': 0.25,
 'support': 20}
weighted
{'f1-score': 0.34102564102564104,
 'precision': 0.5375,
 'recall': 0.25,
 'support': 20}
```

## javascript
### Summary
2 rules, avg.len. 2.5

| | |
|-|-|
|Min support|132|
|Max support|138|
|Min confidence|0.9239130616188049|
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
                     'max_depth': 10,
                     'min_samples_leaf': 95,
                     'min_samples_split': 203,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -1.reserved = :<br>⇒ y = ␣<br>Confidence: 0.996. Support: 132.` |
| 2 | `  -1.reserved not in {:}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +2.reserved not in {:}<br>	∧ ^1.roles in {LITERAL}<br>⇒ y = ∅<br>Confidence: 0.924. Support: 138.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 2.5, "max_conf": 0.9962121248245239, "max_support": 138, "min_conf": 0.9239130616188049, "min_support": 132, "num_rules": 2}}
```
</details>
