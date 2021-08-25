# Model report for file:///tmp/top-repos-quality-repos-v58ecu1b/zerotools.git HEAD c9132dcf3d43963014c065fc4cbada2ba247cec3

### Dump

```json
{'created_at': '2021-08-21 10:38:28',
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
 'size': '16.1 kB',
 'tags': [],
 'uuid': '7b6b94b1-bcee-4e33-af8a-04dd4345552d',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-v58ecu1b/zerotools.git c9132dcf3d43963014c065fc4cbada2ba247cec3

# javascript
24 rules, avg.len. 7.4
## train
PPCR: 0.955367
### report
macro
{'f1-score': 0.9134158394743583,
 'precision': 0.9355139242908337,
 'recall': 0.8954649014540309,
 'support': 27591}
micro
{'f1-score': 0.9541517161393208,
 'precision': 0.9541517161393208,
 'recall': 0.9541517161393208,
 'support': 27591}
weighted
{'f1-score': 0.9539776943112642,
 'precision': 0.9551680464690395,
 'recall': 0.9541517161393208,
 'support': 27591}
### report_full
macro
{'f1-score': 0.8649714711091893,
 'precision': 0.9355139242908337,
 'recall': 0.8150163474124194,
 'support': 28880}
micro
{'f1-score': 0.932372368118149,
 'precision': 0.9541517161393208,
 'recall': 0.9115650969529085,
 'support': 28880}
weighted
{'f1-score': 0.9301533954327265,
 'precision': 0.9540137798463939,
 'recall': 0.9115650969529085,
 'support': 28880}
## test
PPCR: 0.957746
### report
macro
{'f1-score': 0.7346678798908098,
 'precision': 0.7142857142857143,
 'recall': 0.7702237521514631,
 'support': 136}
micro
{'f1-score': 0.9044117647058824,
 'precision': 0.9044117647058824,
 'recall': 0.9044117647058824,
 'support': 136}
weighted
{'f1-score': 0.9017796927688274,
 'precision': 0.9142156862745099,
 'recall': 0.9044117647058824,
 'support': 136}
### report_full
macro
{'f1-score': 0.718341349278565,
 'precision': 0.7142857142857143,
 'recall': 0.7345094664371773,
 'support': 142}
micro
{'f1-score': 0.8848920863309353,
 'precision': 0.9044117647058824,
 'recall': 0.8661971830985915,
 'support': 142}
weighted
{'f1-score': 0.8660908123902652,
 'precision': 0.8802816901408451,
 'recall': 0.8661971830985915,
 'support': 142}
```

## javascript
### Summary
20 rules, avg.len. 7.2

| | |
|-|-|
|Min support|95|
|Max support|4281|
|Min confidence|0.9292452931404114|
|Max confidence|0.9988962411880493|

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
                     'min_samples_leaf': 91,
                     'min_samples_split': 182,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -1.reserved = (<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.955. Support: 189.` |
| 2 | `  -1.label in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = '<br>Confidence: 0.997. Support: 156.` |
| 3 | `  -1.internal_type = StringLiteral<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = '<br>Confidence: 0.996. Support: 137.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ +3.roles not in {RIGHT}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.930. Support: 3322.` |
| 5 | `  ^1.internal_type = MemberExpression<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 4281.` |
| 6 | `  -1.reserved = ;<br>	∧ +1.reserved = }<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.959. Support: 454.` |
| 7 | `  -1.reserved = ;<br>	∧ +1.reserved not in {}}<br>	∧ +2.length ≥ 15<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎⏎<br>Confidence: 0.932. Support: 242.` |
| 8 | `  -1.reserved not in {;, {}<br>	∧ ^1.internal_type = VariableDeclarator<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.956. Support: 793.` |
| 9 | `  -1.reserved not in {;, {}<br>	∧ -1.roles in {IDENTIFIER}<br>	∧ ^1.internal_type not in {MemberExpression, VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.964. Support: 2760.` |
| 10 | `  -1.reserved not in {;, {}<br>	∧ -1.roles in {STRING} and not in {IDENTIFIER}<br>	∧ ^1.internal_type not in {MemberExpression, VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = '<br>Confidence: 0.999. Support: 453.` |
| 11 | `  -1.reserved not in {;, {}<br>	∧ -1.roles not in {IDENTIFIER, STRING}<br>	∧ +1.reserved = ;<br>	∧ ^1.internal_type not in {MemberExpression, VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 1208.` |
| 12 | `  -1.reserved = (<br>	∧ -1.roles not in {IDENTIFIER, STRING}<br>	∧ +1.roles in {STRING}<br>	∧ ^1.internal_type not in {MemberExpression, VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = '<br>Confidence: 0.994. Support: 239.` |
| 13 | `  -1.reserved = (<br>	∧ -1.roles not in {IDENTIFIER, STRING}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 1320.` |
| 14 | `  -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {IDENTIFIER, STRING}<br>	∧ -2.length ≥ 24<br>	∧ +1.reserved not in {;}<br>	∧ ^1.internal_type not in {MemberExpression, VariableDeclarator}<br>	∧ ^1.roles in {FILE} and not in {OPERATOR}<br>⇒ y = ⏎⏎<br>Confidence: 0.960. Support: 211.` |
| 15 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {IDENTIFIER, STRING}<br>	∧ -2.length ≤ 23<br>	∧ +1.reserved not in {;}<br>	∧ ^1.internal_type not in {MemberExpression, VariableDeclarator}<br>	∧ ^1.roles in {FILE} and not in {OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.946. Support: 212.` |
| 16 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {IDENTIFIER, STRING}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {IDENTIFIER}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {FILE} and not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.942. Support: 95.` |
| 17 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {IDENTIFIER, STRING}<br>	∧ -2.length ≤ 23<br>	∧ +1.reserved not in {(, ;, {, }}<br>	∧ +1.roles not in {IDENTIFIER}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles in {FILE} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.972. Support: 1190.` |
| 18 | `  -1.reserved not in {(, {}<br>	∧ -1.roles not in {IDENTIFIER, STRING}<br>	∧ +1.reserved = )<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {FILE, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 297.` |
| 19 | `  -1.label in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {IDENTIFIER, STRING}<br>	∧ +1.reserved not in {), ;}<br>	∧ ^1.internal_type not in {MemberExpression, VariableDeclarator}<br>	∧ ^1.roles not in {FILE, OPERATOR}<br>⇒ y = '<br>Confidence: 0.997. Support: 148.` |
| 20 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {, }}<br>	∧ -1.roles not in {IDENTIFIER, STRING}<br>	∧ +1.reserved = ,<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {FILE, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.929. Support: 106.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 7.15, "max_conf": 0.9988962411880493, "max_support": 4281, "min_conf": 0.9292452931404114, "min_support": 95, "num_rules": 20}}
```
</details>
