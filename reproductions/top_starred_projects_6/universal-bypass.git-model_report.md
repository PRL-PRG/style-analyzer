# Model report for file:///tmp/top-repos-quality-repos-wzyxmepu/universal-bypass.git HEAD 404d1a46fd22020635a977fdf8767ce707b10a76

### Dump

```json
{'created_at': '2021-08-29 21:28:36',
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
 'size': '15.2 kB',
 'tags': [],
 'uuid': '7994555c-7f4a-44ec-8562-679f4eb15d23',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-wzyxmepu/universal-bypass.git 404d1a46fd22020635a977fdf8767ce707b10a76

# javascript
14 rules, avg.len. 6.3
## train
PPCR: 0.951333
### report
macro
{'f1-score': 0.8744306662844871,
 'precision': 0.9218913317321716,
 'recall': 0.8655410186431687,
 'support': 25158}
micro
{'f1-score': 0.9631528738373479,
 'precision': 0.9631528738373479,
 'recall': 0.9631528738373479,
 'support': 25158}
weighted
{'f1-score': 0.9579230757074051,
 'precision': 0.9601324397245341,
 'recall': 0.9631528738373479,
 'support': 25158}
### report_full
macro
{'f1-score': 0.8368412886953774,
 'precision': 0.9218913317321716,
 'recall': 0.8115457579382314,
 'support': 26445}
micro
{'f1-score': 0.9391314458461717,
 'precision': 0.9631528738373479,
 'recall': 0.9162790697674419,
 'support': 26445}
weighted
{'f1-score': 0.9292698874634205,
 'precision': 0.9578904382064919,
 'recall': 0.9162790697674419,
 'support': 26445}
## test
PPCR: 0.946661
### report
macro
{'f1-score': 0.8273273373488399,
 'precision': 0.8391873964083368,
 'recall': 0.826208777801018,
 'support': 2112}
micro
{'f1-score': 0.9512310606060606,
 'precision': 0.9512310606060606,
 'recall': 0.9512310606060606,
 'support': 2112}
weighted
{'f1-score': 0.9462140924742587,
 'precision': 0.9427459753978681,
 'recall': 0.9512310606060606,
 'support': 2112}
### report_full
macro
{'f1-score': 0.7988341479486744,
 'precision': 0.8391873964083368,
 'recall': 0.7759748090495947,
 'support': 2231}
micro
{'f1-score': 0.925166935298181,
 'precision': 0.9512310606060606,
 'recall': 0.9004930524428507,
 'support': 2231}
weighted
{'f1-score': 0.9178061404279948,
 'precision': 0.9388729176503718,
 'recall': 0.9004930524428507,
 'support': 2231}
```

## javascript
### Summary
10 rules, avg.len. 5.7

| | |
|-|-|
|Min support|133|
|Max support|11708|
|Min confidence|0.9213169813156128|
|Max confidence|0.9996150732040405|

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
| 1 | `  -1.internal_type = StringLiteral<br>⇒ y = "<br>Confidence: 1.000. Support: 1299.` |
| 2 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = "<br>Confidence: 0.932. Support: 1400.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {MAP}<br>⇒ y = ⏎⇥⁺<br>Confidence: 0.982. Support: 682.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = )<br>	∧ ^1.roles in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 1166.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = )<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), }}<br>	∧ ^1.roles in {STATEMENT}<br>⇒ y = ⏎<br>Confidence: 0.950. Support: 846.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = (<br>	∧ ^1.roles in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 366.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {(, }}<br>	∧ ^1.roles in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 213.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.roles in {EXPRESSION} and not in {STATEMENT, VALUE}<br>⇒ y = ∅<br>Confidence: 0.972. Support: 11708.` |
| 9 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ -3.label in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {EXPRESSION, STATEMENT, VALUE}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 133.` |
| 10 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {EXPRESSION, STATEMENT, VALUE}<br>⇒ y = ∅<br>Confidence: 0.921. Support: 896.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 5.7, "max_conf": 0.9996150732040405, "max_support": 11708, "min_conf": 0.9213169813156128, "min_support": 133, "num_rules": 10}}
```
</details>
