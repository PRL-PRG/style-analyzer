# Model report for file:///tmp/top-repos-quality-repos-dtiwvh3g/api-sieg-down-v2.git HEAD 6a5fa6ff3c549b224d2cc33757d06184ddda6e5a

### Dump

```json
{'created_at': '2021-08-21 09:49:24',
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
 'size': '15.8 kB',
 'tags': [],
 'uuid': 'a02e1ad4-fb0c-4a9f-9e11-e2ffb1779883',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-dtiwvh3g/api-sieg-down-v2.git 6a5fa6ff3c549b224d2cc33757d06184ddda6e5a

# javascript
9 rules, avg.len. 5.4
## train
PPCR: 0.775658
### report
macro
{'f1-score': 0.5051242918693684,
 'precision': 0.517612983542616,
 'recall': 0.5002443727315729,
 'support': 3537}
micro
{'f1-score': 0.8968052021487136,
 'precision': 0.8968052021487136,
 'recall': 0.8968052021487136,
 'support': 3537}
weighted
{'f1-score': 0.8898717806464642,
 'precision': 0.8872427380819008,
 'recall': 0.8968052021487136,
 'support': 3537}
### report_full
macro
{'f1-score': 0.45874032705709705,
 'precision': 0.517612983542616,
 'recall': 0.42833812112505576,
 'support': 4560}
micro
{'f1-score': 0.783500061751266,
 'precision': 0.8968052021487136,
 'recall': 0.6956140350877194,
 'support': 4560}
weighted
{'f1-score': 0.7431409020640619,
 'precision': 0.809689114166098,
 'recall': 0.6956140350877194,
 'support': 4560}
## test
PPCR: 0.774521
### report
macro
{'f1-score': 0.5260544950738917,
 'precision': 0.5180373464599869,
 'recall': 0.5359807691549234,
 'support': 687}
micro
{'f1-score': 0.901018922852984,
 'precision': 0.901018922852984,
 'recall': 0.901018922852984,
 'support': 687}
weighted
{'f1-score': 0.8815262981227727,
 'precision': 0.8634876690789621,
 'recall': 0.901018922852984,
 'support': 687}
### report_full
macro
{'f1-score': 0.46931283546651376,
 'precision': 0.5180373464599869,
 'recall': 0.44599376976823857,
 'support': 887}
micro
{'f1-score': 0.7865311308767471,
 'precision': 0.901018922852984,
 'recall': 0.6978579481397971,
 'support': 887}
weighted
{'f1-score': 0.72651316073707,
 'precision': 0.7656286037809521,
 'recall': 0.6978579481397971,
 'support': 887}
```

## javascript
### Summary
5 rules, avg.len. 4.8

| | |
|-|-|
|Min support|96|
|Max support|750|
|Min confidence|0.953125|
|Max confidence|0.9961240291595459|

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
| 1 | `  ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.959. Support: 750.` |
| 2 | `  -1.internal_type = StringLiteral<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.996. Support: 129.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {INCOMPLETE} and not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.953. Support: 96.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ), {}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {DECLARATION} and not in {INCOMPLETE, OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.954. Support: 140.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {VARIABLE} and not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.956. Support: 103.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 4.8, "max_conf": 0.9961240291595459, "max_support": 750, "min_conf": 0.953125, "min_support": 96, "num_rules": 5}}
```
</details>
